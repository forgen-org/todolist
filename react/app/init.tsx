"use client"

import { useEffect, useState } from "react"

import init, { onPanic } from "@/wasm"

export default function Init({ children }: { children: React.ReactNode }) {
  const [ready, setReady] = useState(false)

  useEffect(() => {
    ;(async function () {
      await init()
      onPanic()
      setReady(true)
    })()
  })

  return ready ? <>{children}</> : <>Loading...</>
}
