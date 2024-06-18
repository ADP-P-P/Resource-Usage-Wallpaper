<template>
  <div class="timeline">
    <div class="time">{{ time }}</div>
    <div class="date">{{ date }}</div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  data() {
    return {
      time: "-- : -- : --",
      date: "---- / -- / --",
    };
  },
  mounted() {
    setInterval(() => {
      const date = new Date();
      const h = date.getHours();
      const m = date.getMinutes();
      const s = date.getSeconds();
      const Y = date.getFullYear();
      const M = date.getMonth() + 1;
      const d = date.getDate();
      this.time = `${h.toString().padStart(2, "0")} : ${m
        .toString()
        .padStart(2, "0")} : ${s.toString().padStart(2, "0")}`;
      this.date = `${Y.toString().padStart(4, "0")} / ${M.toString().padStart(
        2,
        "0"
      )} / ${d.toString().padStart(2, "0")}`;
    }, 1000);
  },
});
</script>

<style scoped>
.timeline {
  display: flex;
  flex-direction: column;
  align-items: center;
}
.time {
  padding-top: 5%;
  color: white;
  font-size: 4rem;
}
.date {
  color: grey;
  font-size: 2rem;
}
</style>
