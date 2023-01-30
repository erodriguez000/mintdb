import Component from "../../../../lib/component.js";
import mintDB from "../../../../lib/mint.js";
import store from "../../../../store/index.js";

export default class SQLInput extends Component {
    constructor() {
        super({
            store,
            element: document.getElementById("sql-input"),
            subscriptions: []
        });
    }
    render() {
        this.element.innerHTML = /*html*/`
            <div class="sql-input-container">
                <div class="sql-input-group">
                    <div class="sql-input-group-container">
                        <select id="sql-select">
                            <option disabled selected value="">SQL</option>
                            <option value="SELECT">SELECT</option>
                            <option value="CREATE">CREATE</option>
                            <option value="MERGE">MERGE</option>
                            <option value="ADD">ADD TABLE</option>
                            <option value="FIND">FIND</option>
                            <option value="MATCH">MATCH</option>
                            <option value="DELETE">DELETE</option>
                            <option value="PUSH">PUSH</option>
                            <option value="PUT">PUT</option>
                            <option value="REL">REL</option>
                            <option value="BFS">BFS</option>
                            <option value="DFS">DFS</option>
                        </select>
                        <input type="text" id="tb-input" placeholder="table">
                        <input type="text" id="doc-input" placeholder="document">
                        <button class="sql-send-btn" id="sql-send">
                            <svg xmlns="http://www.w3.org/2000/svg" class="sql-send-icon" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                                <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                                <line x1="10" y1="14" x2="21" y2="3"></line>
                                <path d="M21 3l-6.5 18a0.55 .55 0 0 1 -1 0l-3.5 -7l-7 -3.5a0.55 .55 0 0 1 0 -1l18 -6.5"></path>
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        `;

        document.querySelector("#sql-send").addEventListener("click", async () => {
            const data = this.parseData();
            let payload;
            try {
                payload = await mintDB.customSql(data);
            } catch (e) {
                console.log(e.message);
                payload = e.message;
            }
            store.dispatch('fetchQuery', payload);
        });
    }
    parseData() {
        const stmt = document.querySelector("#sql-select").value;
        const tb = document.querySelector("#tb-input").value;
        const doc = document.querySelector("#doc-input").value;
        const json = document.querySelector("#sql-json-input").value;
        let data;
        try {
            data = JSON.parse(json);
        } catch (error) {
            store.dispatch('fetchQuery', error);
        }
        return {
            stmt: stmt,
            tb: tb,
            doc: doc,
            data,
            topic: "",
            user_id: 1,
            message: ""    
        };
    }
}