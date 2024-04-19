<script>
import { ref, resolveTransitionHooks } from 'vue';
import { warsztaty_rust_backend } from 'declarations/warsztaty_rust_backend/index';
</script>

<template>
  <main>
    <router-view /> 
    <!-- all other code of page is inside "views" folder (usage of router mechanism) -->
  </main>
</template>