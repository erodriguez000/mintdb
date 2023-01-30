import Component from "../../../../lib/component.js";
import TableList from "./TableList.js";
import store from "../../../../store/index.js";
import mintDB from "../../../../lib/mint.js";

export default class TableIndex extends Component {
    constructor() {
        super({
            store,
            element: document.getElementById("tb-index"),
            subscriptions: [],
        });
    }

    async render() {
        this.element.innerHTML = /*html*/`
            <div class="table-list-header">
                <input type="text" class="add-table-input" id="filter-tables" placeholder="find table">
                <button class="add-table-btn" id="filter-tables-btn">
                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                        <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                        <circle cx="10" cy="10" r="7"></circle>
                        <line x1="21" y1="21" x2="15" y2="15"></line>
                    </svg>
                </button>    
            </div>
            <div class="table-list-container" id="tb-container"></div>
            <div class="add-table-container">
                <input type="text" class="add-table-input" id="add-table-input" placeholder="add table">
                <button class="add-table-btn" id="add-tb-btn">
                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                        <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                        <line x1="12" y1="5" x2="12" y2="19"></line>
                        <line x1="5" y1="12" x2="19" y2="12"></line>
                    </svg>
                </button>
            </div>
        `;

        const tableList = new TableList();
        await tableList.render();
        
        document.querySelector("#filter-tables-btn").addEventListener("click", (e) => {
            const filter = document.querySelector("#filter-tables");
            console.log(filter.value);
            filter.value = "";
        });
        document.querySelector("#add-tb-btn").addEventListener("click", async () => {
            const newTb = document.querySelector("#add-table-input");
            await mintDB.createTable(newTb.value);
            newTb.value = "";
            const payload = await mintDB.getTableList();
            store.dispatch('fetchTables', payload);
        });
    }
}