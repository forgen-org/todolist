import { makeAutoObservable, runInAction } from "mobx"

import { Home, Task } from "@/wasm"

export class HomeView {
  public description = ""
  public tasks: Task[] = []

  constructor(private presenter = new Home()) {
    console.log(this.presenter)
    makeAutoObservable(this)
    this.listTasks()
  }

  public async listTasks() {
    const { tasks } = await this.presenter.listTasks()
    runInAction(() => {
      this.tasks = tasks
    })
  }

  public async addTask() {
    if (this.description !== "") {
      await this.presenter.addTask(this.description)
      this.changeFormDescription("")
    }
  }

  public changeFormDescription(description: string) {
    this.description = description
  }
}
