"use client"

import { useEffect, useMemo, useState } from "react"

import { Home, Task } from "@/wasm/js"

const Index = () => {
  const home = useMemo(() => new Home(), [])
  const [value, setValue] = useState("")
  const [tasks, setTasks] = useState<Task[]>([])

  useEffect(() => {
    const init = async () => {
      const { tasks } = await home.listTasks()
      setTasks(tasks)
    }
    init()
  }, [])

  const onSubmit = async () => {
    if (value !== "") {
      await home.addTask(value)
      const { tasks } = await home.listTasks()
      setTasks(tasks)
      setValue("")
    }
  }

  return (
    <div>
      <ul>
        {tasks.map((task) => (
          <li key={task.description}>
            <input type="checkbox" checked={task.done} readOnly />
            {task.description}
          </li>
        ))}
      </ul>
      <input type="text" value={value} onChange={(e) => setValue(e.currentTarget.value)} />
      <button onClick={onSubmit}>Add</button>
    </div>
  )
}

export default Index
