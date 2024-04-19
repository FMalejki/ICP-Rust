<script>
import { ref, resolveTransitionHooks } from 'vue';
import { warsztaty_rust_backend } from 'declarations/warsztaty_rust_backend/index';
import 'vue-router';
import router from '../router/index.js';

export default {
  data() {
    return {
      greeting: '',
      wpisy: [],
      nowyWpis: '',
      usrName: [],
      desc: [],
      userName: ''
      //n: wpisy.length
    }
  },
  methods: {
    async handleSubmit(e) {
      e.preventDefault();
      const target = e.target;
      const name = target.querySelector('#name').value;
      await warsztaty_rust_backend.greet(name).then((response) => {
        this.greeting = response;
      });
    },
    async pobierzWpisyOldOne() {
      const wpisy = await warsztaty_rust_backend.pobierz_wpisy()
      this.wpisy = wpisy
    },
    //modified accessing data from blockchain:
    async pobierzWpisy(){
      const allData = await warsztaty_rust_backend.pobierz_wpisy();
      //processing data
      let i = 0;
      let usr = [];
      let desc = [];
      const allDataLen = allData.length;
      while(i < allDataLen){
        let posOfFirstEnd = allData[i].indexOf(";");
        let posOfSeparator = posOfFirstEnd + 1;
        usr.push(allData[i].substr(7,posOfFirstEnd-7));
        desc.push(allData[i].substr(posOfSeparator+6));
        i++;
      }
      //USR_NM=Filip;&
      //begin with 7 and 
      //; is on 12
      //num of characters to get is 12-7 so 5 (posOfFirstEnd-7)
      this.usrName = usr;
      this.desc = desc;
    },

    async dodajWpis() {
      if (this.nowyWpis.trim() === "" || this.userName.trim() === "") return;
      //here the data is going to be transformed in order to fit smoothly into Blockchain Structure
      const pos0 = this.userName.indexOf(";");
      const pos1 = this.userName.indexOf("&");
      if(pos0 != -1 || pos1 != -1){
        alert("Nie używaj znaków specjalnych!"); 
        return;
      }
      let Data = "USR_NM="+this.userName.trim()+";"+"&"+"POST="+this.nowyWpis.trim()
      console.log(Data)
      await warsztaty_rust_backend.dodaj_wpis(Data)
      console.log("dodano nowy wpis!")
      await this.pobierzWpisy()
      this.nowyWpis = '';
      this.userName = '';
    },

    async whtsActv(routeName) {
      //this is just not working concept but an option for future
      //part of code in router-link in header , 'active': whtsActv('/login')
        console.log("?",routeName)
        console.log("ans",this.$route.name)
        console.log("return ",this.$route.path === routeName)
        return this.$route.path === routeName
      }
  },

  async mounted() {
      await this.pobierzWpisy()
  },
};
</script>

<template>
    <header class="headerMod navbar navbar-expand-lg navbar-dark bg-dark">
    <div class="container">
        <a class="navbar-brand" href="#"><img src="/logo2.svg" alt="DFINITY logo" class="img-fluid" style="max-width: 100px;"/></a>
        <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarNav" aria-controls="navbarNav" aria-expanded="false" aria-label="Toggle navigation">
        <span class="navbar-toggler-icon"></span>
        </button>
        <div class="collapse navbar-collapse" id="navbarNav">
        <ul class="navbar-nav ms-auto">
            <li class="nav-item">
            <router-link id="home" :class="{'nav-link' : true, 'active' : true}" to="/">Strona Główna</router-link>
            </li>
            <li class="nav-item">
            <router-link id="login" :class="{ 'nav-link': true, 'active' : false}" to="/login">Logowanie</router-link>
            </li>
        </ul>
        </div>
    </div>
    </header>
    <div class="firstElement center mt-4">
        <h1>Witaj na Blogu w 100% Opartym Na Technologii Blockchain!</h1>
        <p>Udostępnij swój pierwszy wpis!</p>
    </div>
    <div class="d-flex justify-content-between flex-wrap flex-md-nowrap align-items-center pt-3 pb-2 mb-1 border-bottom border-3 border-black"></div>
    <div class="center container mt-4 mb-4">
      <h2 class="mb-4" >Ostatnie publikacje!</h2>
      <div class="w-100 center container">
        <div class="row">
          <div class="col">
            <div class="center p-4 mt-10 boxSecond border border-black border-2 rounded">
              <h3>{{ usrName[usrName.length - 2] }}</h3>
              <div class="textInsideBoxSecond">{{  desc[desc.length - 2] }}</div>
            </div>
          </div>
          <div class="col">
            <div class="center p-4 mt-10 box border border-black border-2 rounded">
              <h3>{{ usrName[usrName.length - 1] }}</h3>
              <div class="textInsideBox">{{  desc[desc.length - 1] }}</div>
            </div>
          </div>
          <div class="col">
            <div class="center p-4 mt-10 boxSecond border border-black border-2 rounded">
              <h3>{{ usrName[usrName.length - 3] }}</h3>
              <div class="textInsideBoxSecond">{{  desc[desc.length - 3] }}</div>
            </div>
          </div>
        </div>
      </div>
    </div>
    <div class="mb-10 d-flex justify-content-between flex-wrap flex-md-nowrap align-items-center pt-3 pb-2 mb-1 border-bottom border-3 border-black"></div>
    <div class="mt-10 mb-10 container center">
      <br />
      <h2 class="mt-10">Dodaj swój własny post!</h2>
      <div class="container center">
        <div class="container center">
          <br />
          <h3>Podaj swoją nazwę użytkownika:</h3>
          <br />
          <input v-model="userName">
        </div>
        <div class="mt-10 container center">
          <br />
          <h3 class="mt-10">Tutaj wpisz treść posta:</h3>
          <br />
          <textarea rows="5" cols="100" v-model="nowyWpis"></textarea>
        </div>
        <br />
        <div class="mt-10">
          <button class="center" @click="dodajWpis()">Wrzuć post na sieć Blockchain!</button>
        </div>
        <br />
        <br />
      </div>
    </div>
</template>