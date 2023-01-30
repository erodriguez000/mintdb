import Component from "../../../../lib/component.js";
import store from "../../../../store/index.js";
import DocumentDetail from "../doc/DocumentDetail.js";
import DocumentList from "../doc/DocumentList.js";
import TableHeader from "./TableHeader.js";

export default class TableDetail extends Component {
    constructor() {
        super({
            store,
            element: document.getElementById("tb-detail"),
            subscriptions: []
        });

    }
    async render() {
        this.element.innerHTML = /*html*/`
            <div class="table-detail-header" id="tb-header"></div>
            <div class="document-detail-view">
                <div class="document-index-container" id="doc-list"></div>
                <div class="document-details" id="doc-detail"></div>
            </div>
        `;
        const tableHeader = new TableHeader();
        await tableHeader.render();

        const documentList = new DocumentList();
        documentList.render();
        
        const documentDetail = new DocumentDetail();
        documentDetail.render();
    }
}