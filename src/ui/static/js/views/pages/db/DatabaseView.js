import Component from "../../../lib/component.js";
import TableDetail from "./tb/TableDetail.js";
import store from "../../../store/index.js";
import TableIndex from "./tb/TableIndex.js";
import mintDB from "../../../lib/mint.js";

export default class DatabaseView extends Component {
    constructor() {
        super({
            store,
            element: document.getElementById("content"),
        });
    }
    async render() {
        this.element.innerHTML = /*html*/`
            <div class="table-index-container" id="tb-index"></div>
            <div class="table-view-container" id="tb-detail"></div>
        `;
        const payload = await mintDB.getTableList();
        store.dispatch('fetchTables', payload);
        
        const tableIndex = new TableIndex();
        await tableIndex.render();

        const tableDetail = new TableDetail();
        await tableDetail.render();
    }
}