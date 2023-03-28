import { useEffect, useRef, useState } from 'react'

import { Canvas, useFrame } from '@react-three/fiber'

import { Mesh } from 'three'

function WireframeBox(props: any) {
  const mesh = useRef<Mesh>(null!)

  useFrame(() => {
    if (mesh.current) {
      mesh.current.rotation.y += 0.01
      mesh.current.rotation.x = Math.cos(mesh.current.rotation.y) * 0.5
    }
  })

  return (
    <mesh {...props} ref={mesh}>
      <boxBufferGeometry args={[1, 1, 1]} />
      <meshBasicMaterial color={props.color} wireframe />
    </mesh>
  )
}

export default function WireframeBoxComponent() {
  return (
    <Canvas>
      <WireframeBox position={[0, 0, 0]} scale={[3, 3, 3]} color={'#aaa'} />
    </Canvas>
  )
}
