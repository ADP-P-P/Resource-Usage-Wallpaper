<template>
  <div class="container" v-if="net">
    <div class="title">
      <div class="key">{{ path.toUpperCase() }}</div>
      <div class="last" :key="usage">{{ usage() }}</div>
    </div>
    <TransitionGroup name="list" tag="div" class="blist">
      <div
        class="bar"
        v-for="i in bars"
        :key="i.key"
        :style="{ height: i.value + '%' }"
      ></div>
    </TransitionGroup>
  </div>
  <div v-if="!net" class="error">
    Backend error, please check backend process
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { request } from "../scripts/net";
export default defineComponent({
  props: {
    path: {
      type: String,
      require: true,
    },
    total: {
      type: Number,
      require: true,
    },
  },
  data() {
    return {
      bars: [{ key: 0, value: 0 }],
      net: true,
    };
  },
  mounted() {
    var count = 0;
    setInterval(() => {
      request(this.$props.path!)
        .then((used) => {
          this.net = true;
          count++;
          var value = (used * 100) / this.$props.total!;
          this.bars.push({
            key: count,
            value: value,
          });
          if (this.bars.length > 14) {
            this.bars.splice(0, 1);
          }
        })
        .catch(() => {
          this.net = false;
        });
    }, 2000);
  },
  methods: {
    usage() {
      let bars = this.$data.bars;
      return bars[bars.length - 1].value.toFixed(2).concat("%");
    },
  },
});
</script>

<style scoped>
.container {
  width: 100%;
  height: 40%;
  display: block;
  flex-direction: column;
  align-items: end;
}
.error {
  color: rgb(252, 159, 159);
  font-size: 1rem;
}
.title {
  width: 100%;
  height: 10%;
  display: flex;
  flex-direction: row;
  justify-content: space-between;
}


.blist {
  height: 90%;
  width: 100%;
  display: flex;
  flex-direction: row;
  flex-wrap: nowrap;
  justify-content: flex-end;
  align-items: flex-end;
  align-content: stretch;
  overflow: scroll;
}

.bar {
  background-color: rgb(228, 228, 228);
  border-radius: 4px;
  height: 0px;
  width: 5%;
  margin-left: 5px;
  height: 100%;
}
.last,
.key {
  display: block;
  color: rgb(228, 228, 228);
}
@keyframes enhight {
  100% {
    height: 100%;
  }
}

.list-move, /* 对移动中的元素应用的过渡 */
  .list-enter-active,
  .list-leave-active {
  transition: all 0.5s ease;
}

.list-enter-from,
.list-leave-to {
  opacity: 0;
  /* transform: translateX(30px); */
}

/* 确保将离开的元素从布局流中删除
    以便能够正确地计算移动的动画。 */
/* .list-leave-active {
} */
</style>
