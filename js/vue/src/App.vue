<template>
  <h1>Counter: {{ count }}</h1>
  <button @click="increment">Increment</button>
  <button @click="decrement">Decrement</button>
</template>

<script setup>
  import {ref} from "vue";

  const count = ref(0);

  function increment()
  {
    count.value++;
  }

  function decrement()
  {
    count.value--;
  }
</script>