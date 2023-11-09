import {
  IonButton,
  IonButtons,
  IonContent,
  IonHeader,
  IonItem,
  IonModal,
  IonTextarea,
  IonTitle,
  IonToolbar,
} from "@ionic/react"
import { useRef } from "react"

import { createTask } from "../wasm"

export const CreateTaskModal = (props: { trigger: string }) => {
  const modal = useRef<HTMLIonModalElement>(null)
  const textarea = useRef<HTMLIonTextareaElement>(null)

  const autofocus = () => {
    textarea.current?.setFocus()
  }

  const confirm = async () => {
    const value = textarea.current?.value
    if (typeof value === "string" && value.length > 0) {
      await createTask(value)
      modal.current?.dismiss()
    }
  }

  return (
    <IonModal ref={modal} trigger={props.trigger} onDidPresent={autofocus}>
      <IonHeader>
        <IonToolbar>
          <IonButtons slot="start">
            <IonButton onClick={() => modal.current?.dismiss()}>Cancel</IonButton>
          </IonButtons>
          <IonTitle>Create a Task</IonTitle>
          <IonButtons slot="end">
            <IonButton strong={true} onClick={confirm}>
              Confirm
            </IonButton>
          </IonButtons>
        </IonToolbar>
      </IonHeader>
      <IonContent className="ion-padding">
        <IonItem>
          <IonTextarea
            aria-label="Description"
            autoGrow={true}
            counter={true}
            label="Description"
            labelPlacement="floating"
            maxlength={200}
            placeholder="Give Todolist 5 stars"
            ref={textarea}
          />
        </IonItem>
      </IonContent>
    </IonModal>
  )
}
