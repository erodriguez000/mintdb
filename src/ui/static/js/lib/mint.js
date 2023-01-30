class MintDB {
    constructor(url, subscriptions = []) {
        let self = this;
        self.url = url;
        self.subscriptions = subscriptions;
        self.ws = null;
    }
    async signup(username, password) {
        const endpoint = "/auth";
        const data = {
            event: "signup",
            username,
            password
        };
        try {
            const res = await this.httpRequest(data, endpoint);
            localStorage.setItem("jwt", res.token);
        } catch (error) {
            console.log(error);
        }
    }
    async signin(username, password) {
        const endpoint = "/auth";
        const data = {
            event: "signin",
            username,
            password
        };
        try {
            const res = await this.httpRequest(data, endpoint);
            localStorage.setItem("jwt", res.token);
        } catch (error) {
            console.log(error);
        }
    }
    async signout(username) {
        const endpoint = "/auth";
        const data = {
            event: "signout",
            username,
        };
        try {
            const res = await this.httpRequest(data, endpoint);
            localStorage.removeItem("jwt");
        } catch (error) {
            console.log(error);
        }
    }
    async registerWebSocket() {
        const data = {
            user_id: 1,
        }
        const res = await fetch(this.url + "/register", {
            method: "POST",
            headers: {
                Accept: "application/json",
                "Content-Type": "application/json",
            },
            body: JSON.stringify(data)
        });
        return await res.json();
    }
    async listen() {
        const { url } = await this.registerWebSocket();
        console.log(url);
        this.ws = new WebSocket(url);
        console.log("listening!");
    }
    onEvent(cb) {
        this.ws.onmessage = function(msg) {
            cb(msg);
        }
    }
    async listenOn(subscriptions, callback) {
        const { url } = await this.registerWebSocket();
        this.ws = new WebSocket(url);

        this.ws.onmessage = function(msg) {
            callback(msg);
        }
    }
    addSubscription(sub) {
        this.subscriptions.push(sub);
        this.updateSubcriptionList();
    }
    removeSubscription(subscription) {
        this.subscriptions = this.subscriptions.filter(sub => sub != subscription);
        this.updateSubcriptionList();
    }

    updateSubcriptionList() {
        let data = {
          "topics": this.subscriptions
        }
        this.ws.send(JSON.stringify(data));
    }
    closeWS() {
        this.ws.close(1000, "User Disconnect");
    }
    async publish(topic, user_id, msg) {
        const data = { topic, user_id, msg };
        const res = await fetch(this.url + "/publish", {
            method: "POST",
            headers: {
                Accept: "application/json",
                "Content-Type": "application/json",
            },
            body: JSON.stringify(data)
        });
        return await res.json();
    }
    async getTableList() {
        const data = {
            stmt: "INFO",
            tb: "",
            doc: "",
            data: {},
            topic: "",
            user_id: 1,
            message: "" 
        }
        return await this.httpRequest(data); 
    }
    async createTable(table) {
        const data = {
            stmt: "ADD",
            tb: table,
            doc: "",
            data: {},
            topic: "",
            user_id: 1,
            message: ""    
        };
        return await this.httpRequest(data); 
    }

    async addDoc(tb, doc, docData) {
        const data = {
            stmt: "CREATE",
            tb: tb,
            doc: doc,
            data: docData,
            topic: "",
            user_id: 1,
            message: ""    
        }
        return this.httpRequest(data);
    }
    async merge(tb, doc, docData) {
        const data = {
            stmt: "MERGE",
            tb: tb,
            doc: doc,
            data: docData,
            topic: "",
            user_id: 1,
            message: ""    
        }
        console.log(data);
        return this.httpRequest(data);
    }
    async getOne(table, doc) {
        const data = {
            stmt: "SELECT",
            tb: table,
            doc: doc,
            data: {},
            topic: "",
            user_id: 1,
            message: ""    
        };
        return await this.httpRequest(data);
    }
    async getAll(table) {
        const data = {
            opts: "",
            sql: "",
            data: {},
            stmt: "SELECT",
            tb: table,
            doc: "*",
            topic: "",
            user_id: 1,
            message: ""    
        };
        return await this.httpRequest(data);
    }

    async find(tb, filters) {
        // filters = {"city": "Clearwater"}
        const data = {
            stmt: "FIND",
            tb: tb,
            doc: "",
            data: filters,
            topic: "",
            user_id: 1,
            message: ""
        };
        return await this.httpRequest(data);
    }
    async addEdge(tb, edge) {
        const data = {
            stmt: "EDGE",
            tb: tb,
            data: edge,
            doc: "",
            topic: "",
            user_id: 1,
            message: ""
        };
        return await this.httpRequest(data);
    }
    async customSql(data) {
        const res = await this.httpRequest(data);
        return JSON.stringify(res);
    }
    async httpRequest(data = {}, endpoint = "/sql") {
        const res = await fetch(this.url + endpoint, {
            method: "POST",
            headers: {
                Authorization: `Bearer ${localStorage.getItem("jwt") ?? ""}`,
                Accept: "application/json",
                "Content-Type": "application/json",
            },
            body: JSON.stringify(data)
        });
        return await res.json();
    }
}

export default new MintDB("http://127.0.0.1:8000");