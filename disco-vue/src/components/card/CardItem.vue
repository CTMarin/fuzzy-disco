<template>
  <div class="card-wrap">
    <div class="card">
      <div :style="cssVars" class="card-bg"></div>
      <h1 class="card-title">{{ card.title }}</h1>
      <PlayComp></PlayComp>
      <Slider :card="card" class="slider-comp"></Slider>
    </div>
  </div>
</template>

<script>
import Slider from "@/components/card/Slider";
import PlayComp from "@/components/card/PlayComp";

export default {
  name: "CardItem",
  components: {PlayComp, Slider},
  props: ['card'],
  computed: {
    cssVars() {
      return  {
        '--card-background': "url(" + this.loadData() + ")",
      }
    }
  },
  methods: {
    async loadData() {
      console.log("ok")
      let response = await fetch(`/api/media/${this.card.photo}`);
      console.log(response.ok);
      if(response.ok) {
        console.log(response);
        return response;
      }
    }
  }
}
</script>

<style scoped>
  .card-wrap {
    margin: 30px;
    padding: 0;
    border: 0;
  }

  .card {
    position: relative;
    border-radius: 35px;
    height: 320px;
    width: 240px;
    overflow: hidden;
    box-shadow: 0 0 3px 0 #ddd;
    transition: 350ms;
  }

  .card-bg {
    --card-background: "";
    position: absolute;
    opacity: 0.5;
    height: 100%;
    width: 100%;
    top: -20px;
    left: -20px;
    padding: 20px;
    background-image: var(--card-background);
    background-repeat: no-repeat;
    background-position: center;
    background-size: cover;
    transition: 600ms;
  }

  .card-title {
    opacity: 1;
    position: relative;
    margin: 10px;
    color: #eeeeee;
    cursor: default;
    transition: 200ms;
  }

  .slider-comp {
    bottom: -20%;
    opacity: 0;
  }

  .card:hover {
    box-shadow: rgba(100, 100, 100, 0.66) 0 30px 60px -10px, inset whitesmoke 0 0 0 5px;
    outline-offset: 0;
  }

  .card:hover .card-bg {
    filter: blur(4px) brightness(80%);
    padding: 0;
    top: 0;
    left: 0;
  }

  .card:hover .slider-comp {
    opacity: 1;
    bottom: 0;
  }

  .card:hover .card-title {
    opacity: 0;
  }

</style>