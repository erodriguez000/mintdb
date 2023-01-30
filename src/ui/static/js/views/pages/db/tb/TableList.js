import Component from "../../../../lib/component.js";
import mint from "../../../../lib/mint.js";
import store from "../../../../store/index.js";
export default class TableList extends Component {
    constructor() {
        super({
            store,
            element: document.getElementById("tb-container"),
            subscriptions: ["tables", "activeTable"]
        });
    }

    async render() {
        this.element.innerHTML = /*html*/`
        ${store.state.tables.map(tb => {
            return /*html*/`
            <div class="table-list-link ${tb === store.state.activeTable && "active"}" id="${tb}">
                <div class="table-list-item">
                    <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-table" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                        <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                        <rect x="4" y="4" width="16" height="16" rx="2"></rect>
                        <line x1="4" y1="10" x2="20" y2="10"></line>
                        <line x1="10" y1="4" x2="10" y2="20"></line>
                    </svg>
                    <p>${tb.length > 10 ? `${tb.slice(0, 10)}...` : tb}</p>
                </div>
            </div>
            `
        }).join('')}
        `
        const links = document.querySelectorAll(".table-list-link");
        links.forEach(ln => ln.addEventListener("click", async () => {
            const activeTable = ln.id;
            const documents = await mint.getAll(activeTable);
            store.dispatch("setDocuments", documents);
            store.dispatch("setActiveTable", activeTable);
        }));
    }
}