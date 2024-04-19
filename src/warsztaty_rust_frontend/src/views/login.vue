<script lang="js">
    import { defineComponent } from 'vue';
    import { warsztaty_rust_backend } from "declarations/warsztaty_rust_backend/index";

    export default defineComponent( {
    data() {
        return {
        logPanel: true,
        regPanel: false,
        addUserUsername: '',
        addUserFirstPasswd: '',
        addUserSecondPasswd: '',
        samePasswdWarn: false,
        emptyDataWarn: false,
        testData: ''
        };
    },
    methods: {
        swapPanels() {
            this.logPanel = !this.logPanel;
            this.regPanel = !this.regPanel;
        },
        async addUser() {
            if(this.addUserUsername === '' || this.addUserFirstPasswd === '' || this.addUserSecondPasswd === ''){
                this.samePasswdWarn = false;
                this.emptyDataWarn = true;
                return;
            }
            if(this.addUserFirstPasswd != this.addUserSecondPasswd){
                this.emptyDataWarn = false;
                this.samePasswdWarn = true;
                return;
            }
            this.samePasswdWarn = false;
            this.emptyDataWarn = false;
            let passwd = this.addUserFirstPasswd;// : string
            let userName = this.addUserUsername;//: string 
            //at this moment we are 100% sure that data is inserted properly
            //we can start to insert it into blockchain using RUST functions from backend files            
            await warsztaty_rust_backend.add_user(userName, passwd);
            await this.test_if_passwords_work()
        },
        //Testing component:
        async test_if_passwords_work() {
        const testData = await warsztaty_rust_backend.test_list_passwords()
        this.testData = testData
        },
    },
    async mounted() {
        const arr = await warsztaty_rust_backend.test_list_passwords()
        console.log(arr)
    },
    });
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
            <router-link id="home" :class="{'nav-link' : true, 'active' : false}" to="/">Strona Główna</router-link>
            </li>
            <li class="nav-item">
            <router-link id="login" :class="{ 'nav-link': true, 'active' : true}" to="/login">Logowanie</router-link>
            </li>
        </ul>
        </div>
    </div>
    </header>
    <div class="container">
        <div class="firstElement mt-4 center">
            <h1>ZALOGUJ SIĘ DO SIECI BLOCKCHAIN</h1>
            <p> lub dołącz do grona użytkowników bloga! </p>
        </div>
        <div class="container center">
            <div v-if="logPanel" class=" container center ">
                <div>
                    <input class="mt-5" placeholder="Nazwa użytkownika"/>
                </div>
                <div>
                    <input type="password" class="mt-4" placeholder="Hasło"/>
                </div>
                <div>
                    <button  class="mt-3 rounded">ZALOGUJ</button>
                </div>
                <div>
                    <button  @click="swapPanels" class="mt-2 rounded">ZAREJESTRUJ SIĘ</button>
                </div>
            </div>
            <div v-if="regPanel" class=" container center ">
                <div>
                    <h3 class="mt-5">Podaj swoją nazwę użytkownika:</h3>
                    <input v-model="addUserUsername" placeholder="Nazwa użytkownika"/>
                </div>
                <div>
                    <h3 class="mt-4">Podaj swoje hasło:</h3>
                    <input v-model="addUserFirstPasswd" type="password" placeholder="Hasło"/>
                </div>
                <div>
                    <h3 class="mt-4">Powtórz hasło:</h3>
                    <input v-model="addUserSecondPasswd" type="password" placeholder="Hasło"/>
                </div>
                <div>
                    <button @click="addUser" class="mt-3 rounded">ZAREJESTRUJ SIĘ</button>
                </div>
                <div>
                    <button  @click="swapPanels" class="mt-2 rounded">WRÓĆ DO OKNA LOGOWANIA</button>
                </div>
                <h4 v-if="samePasswdWarn" class="alert alert-warning">Hasła nie zgadzają się ze sobą!</h4>
                <h4 v-if="emptyDataWarn" class="alert alert-warning">Uzupełnij brakujące dane!</h4>
            </div>
        </div>
        <div>
            <h1>TESTY</h1>
            <h3>{{ testData[testData.length - 1] }}</h3>
        </div>
    </div>
</template>