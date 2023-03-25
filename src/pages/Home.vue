<template>
  <div id="home">
    <div id="title-container">
      <h1>Pac Man</h1>
    </div>
    <div>
      <h4>Press SPACE to start</h4>
    </div>
  </div>
</template>

<script lang="ts">
import router, {pac_man} from "../app/Router";

export default {
  name: "Home",
  data() {
    return {}
  },
  mounted() {
    window.addEventListener('keydown', (event) => {
      if (event && event.code) {
        switch (event.code.toLowerCase()) {
          case 'space':
            router.push(pac_man).catch((error: string) => console.error(error));
            break
          default:
            break
        }
      }
    })
  },
}
</script>

<style scoped>
#home {
  height: 100%;
  width: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;
}
</style>