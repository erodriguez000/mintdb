import Component from "../../../../lib/component.js";
import store from "../../../../store/index.js";
import mintDB from "../../../../lib/mint.js"

export default class DocumentList extends Component {
    constructor() {
        super({
            store,
            element: document.getElementById("doc-list"),
            subscriptions: ["activeDocument", "tableDocs"]
        })
    }

    async render() {
        this.element.innerHTML = /*html*/`
            <div class="document-list-header">
                <input type="text" class="add-table-input" id="filter-doc" placeholder="find document">
                <button class="add-table-btn" id="filter-doc-btn">
                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                        <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                        <circle cx="10" cy="10" r="7"></circle>
                        <line x1="21" y1="21" x2="15" y2="15"></line>
                    </svg>
                </button>
            </div>
            <div class="document-list-container">
            ${store.state.tableDocs.map(doc => {
                return /*html*/`
                    <div class="document-list-link" id="${doc.id}" data-doc='${JSON.stringify(doc)}' >
                        <div class="document-list-item">
                            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                                <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                                <path d="M14 3v4a1 1 0 0 0 1 1h4"></path>
                                <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2z"></path>
                                <path d="M9 14v.01"></path>
                                <path d="M12 14v.01"></path>
                                <path d="M15 14v.01"></path>
                            </svg>
                            <p>${doc.id.length > 10 ? `${doc.id.slice(0, 10)}...` : doc.id}</p>
                        </div>
                    </div>
                `
            }).join("")}
            </div>
            <div class="add-doc-container">
                <input type="text" class="add-table-input" id="add-doc-input" placeholder="add document">
                <button class="add-table-btn" id="add-doc-btn">
                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                        <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                        <line x1="12" y1="5" x2="12" y2="19"></line>
                        <line x1="5" y1="12" x2="19" y2="12"></line>
                    </svg>
                </button>
            </div>
        `;
        const activeLink = document.getElementById(`${store.state.activeDocument.id}`);
        if(activeLink) {
            activeLink.classList.add("active");
        }
        
        const documentLinks = this.element.querySelectorAll(".document-list-link");
        documentLinks.forEach(link => link.onclick = () => {
            const activeDocument = JSON.parse(link.dataset.doc);
            store.dispatch("setActiveDocument", { activeDocument });
        });

        document.querySelector("#filter-doc-btn").addEventListener("click", () => {
            const filter = document.querySelector("#filter-doc");
            console.log(filter.value);
            filter.value = "";
        });

        document.querySelector("#add-doc-btn").addEventListener("click", async () => {
            const newDocInput = document.querySelector("#add-doc-input");
            await mintDB.addDoc(store.state.activeTable, `${store.state.activeTable}:${newDocInput.value}`, {});
            const payload = await mintDB.getAll(store.state.activeTable);
            store.dispatch('setDocuments', payload);
            newDocInput.value = "";
        });
    }
}