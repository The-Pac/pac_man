import {createRouter, createWebHistory, RouteRecordRaw} from "vue-router";

export const home: RouteRecordRaw = {
    path: '/',
    name: 'Home',
    component: () => import("../pages/Home.vue")
}
export const pac_man: RouteRecordRaw = {
    path: '/',
    name: 'PacMan',
    component: () => import("../pages/PacMan.vue")
}

export const error_page: RouteRecordRaw = {
    path: '/:pathMatch(.*)*',
    name: 'Error-Page',
    component: () => import("../pages/Error.vue")
}


const routes: Array<RouteRecordRaw> = [
    home,
    pac_man,
    error_page
]

const router = createRouter({
    history: createWebHistory(),
    routes
});

export default router;
