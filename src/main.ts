import {createApp} from "vue";
import "./app/bootstrap.scss";
import App from "./app/App.vue";
import router from "./app/Router";
import {addIcons, OhVueIcon} from "oh-vue-icons";
import {CoExitToApp, CoMediaPlay} from "oh-vue-icons/icons/co"
import {
  FaEdit,
  FaHome,
  FaRegularWindowMaximize,
  FaRegularWindowMinimize
} from "oh-vue-icons/icons/fa";

const app = createApp(App)

//ICONS
addIcons(CoMediaPlay, CoExitToApp,FaEdit,FaHome, FaRegularWindowMaximize, FaRegularWindowMinimize)
app.component("v-icon", OhVueIcon);

//ROUTES
app.use(router)

//MOUNT
app.mount('#app');