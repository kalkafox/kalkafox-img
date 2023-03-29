import { RefObject, useEffect, useRef, useState } from 'react'
import { useTransition, animated as a } from '@react-spring/web'
import CountUp from 'react-countup'

import Cube from './components/Cube'

function App() {
  const [count, setCount] = useState(0)
  const [transitionCount, setTransitionCount] = useState(0)
  const [countUpCount, setCountUpCount] = useState(0)
  const [canvasSize, setCanvasSize] = useState<[number, number]>([
    window.innerWidth,
    window.innerHeight,
  ])
  const canvasRef = useRef<HTMLCanvasElement>(null)

  const transitions = useTransition(transitionCount, {
    from: { opacity: 0 },
    enter: { opacity: 1 },
    leave: { opacity: 0 },
  })

  useEffect(() => {
    setCanvasSize([window.innerWidth, window.innerHeight])
    window.onresize = () => {
      setCanvasSize([window.innerWidth, window.innerHeight])
    }
  }, [])

  useEffect(() => {
    // Draw a subtle grain effect on the entire canvas element (so, multiple dots)
    const canvas = canvasRef.current
    if (canvas) {
      const ctx = canvas.getContext('2d')
      if (ctx) {
        const width = canvas.width
        const height = canvas.height
        const imageData = ctx.createImageData(width, height)
        const data = imageData.data
        for (let i = 0; i < data.length; i += 4) {
          data[i] = 100
          data[i + 1] = 100
          data[i + 2] = 100
          data[i + 3] = Math.random() * 150
        }
        ctx.putImageData(imageData, 0, 0)
      }
    }
  }, [transitionCount])

  return (
    <>
      <div
        onMouseMove={() => {
          if (count % 5 === 0) {
            setTransitionCount(transitionCount + 1)
          }
          if (count % 50 === 0) {
            setCountUpCount(count)
          }
          setCount(count + 1)
        }}
        className='bg-zinc-900 w-full h-full absolute'>
        {transitions((style, item) => (
          <a.canvas
            key={item}
            width={canvasSize[0]}
            height={canvasSize[1]}
            ref={canvasRef}
            style={style}
            className='absolute top-0 left-0 w-full h-full'
          />
        ))}
        <div className='text-center p-2 font-[Poppins] bg-zinc-800/20 backdrop-blur-sm border w-60 rounded-lg text-zinc-300 absolute left-0 right-0 m-auto top-12'>
          <h1 className='font-bold text-4xl'>404</h1>
          <Cube />
          <p>Whoops, that's an error.</p>
        </div>
        <CountUp start={0} end={countUpCount} delay={0} preserveValue={true}>
          {({ countUpRef }) => (
            <div className='fixed text-center p-2 bg-zinc-800/20 backdrop-filter backdrop-blur-lg w-auto rounded-lg text-zinc-300 bottom-0 m-4'>
              <span className='relative font-[Poppins]' ref={countUpRef} />
            </div>
          )}
        </CountUp>
      </div>
    </>
  )
}

export default App
