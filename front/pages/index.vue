<template>
  <div>
    <p>Hello World! {{ toto }}</p>
    <button @click="inc">
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
</script>
