<template>
  <div class="main">
    <div class="dashboard">
      <StatueBar path="mem" :total="memt"></StatueBar>
      <StatueBar path="cpu" :total="100"></StatueBar>
    </div>
    <Timeline></Timeline>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { request } from "./scripts/net";
import StatueBar from "./components/StatueBar.vue";
import Timeline from "./components/Timeline.vue";

export default defineComponent({
  components: {
    StatueBar,Timeline
  },
  data() {
    return {
      memt: -1,
    };
  },
  mounted() {
    request("memt").then((total) => {
      this.$data.memt = total;
    });
  },
});
</script>

<style scoped>
.main {
  height: 100vh;
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  padding: 10%;
}
.dashboard{
  height: 100%;
  width: 40%;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  padding-left: 10vw;
}
</style>
