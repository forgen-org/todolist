import {
  IonButton,
  IonButtons,
  IonContent,
  IonFab,
  IonFabButton,
  IonHeader,
  IonIcon,
  IonPage,
  IonTitle,
  IonToolbar,
} from "@ionic/react"
import { add, play } from "ionicons/icons"

import { CreateTaskModal } from "../features/CreateTaskModal"

export const Home: React.FC = () => {
  return (
    <IonPage>
      <IonHeader>
        <IonToolbar>
          <IonTitle>Todolist</IonTitle>
          <IonButtons slot="end">
            <IonButton id="create-task" expand="block">
              <IonIcon slot="icon-only" icon={add}></IonIcon>
            </IonButton>
          </IonButtons>
        </IonToolbar>
      </IonHeader>
      <IonContent fullscreen>
        <IonFab slot="fixed" vertical="bottom" horizontal="center">
          <IonFabButton>
            <IonIcon icon={play}></IonIcon>
          </IonFabButton>
        </IonFab>
      </IonContent>
      <CreateTaskModal trigger="create-task" />
    </IonPage>
  )
}
