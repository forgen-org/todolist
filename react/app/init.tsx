"use client"

import { useEffect, useState } from "react"

import init, { setPanicHook } from "@/wasm"

export default function Init({ children }: { children: React.ReactNode }) {
  const [ready, setReady] = useState(false)

  useEffect(() => {
    ;(async function () {
      await init()
      setPanicHook()
      setReady(true)
    })()
  })

  return ready ? <>{children}</> : <>Loading...</>
}
