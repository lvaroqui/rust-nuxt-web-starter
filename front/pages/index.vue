<template>
  <div class="container p-4 mx-auto text-gray-800">
    <p>Hello World! {{ toto }}</p>
    <button class="p-2 mx-4 rounded-md bg-slate-500 hover:bg-slate-600" @click="inc">
      Click me
    </button>
    <button class="p-2 mx-4 bg-red-500 rounded-md hover:bg-red-600" @click="dec">
      Click me
    </button>
    <div v-if="pending">
      <p>Loading...</p>
    </div>
    <div v-else>
      {{ data }}
    </div>
  </div>
</template>

<script setup lang="ts">
const toto = ref(0);

useHead(() => ({
  title: "Home Page"
}));

const { pending, data } = useLazyFetch("/api/");

const inc = () => {
  toto.value += 10;
};
const dec = () => {
  toto.value -= 10;
};
</script>
