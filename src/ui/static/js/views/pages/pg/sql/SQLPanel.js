import Component from "../../../../lib/component.js";
import store from "../../../../store/index.js";
import SQLInput from "./SQLInput.js";
import SQLResponse from "./SQLResponse.js";

export default class SQLPanel extends Component {
    constructor() {
        super({
            store,
            element: document.getElementById("sql-panel"),
            subscriptions: []
        });
    }

    async render() {
        this.element.innerHTML = /*html*/`
            <div class="sql-input-view">
                <div class="sql-input" id="sql-input"></div>
            </div>
            <div class="sql-json-container">
                <div class="sql-json-input" id="sqljson">
                    <p style="color: white;font-family: 'Source Code Pro';">JSON Input</p>
                    <textarea spellcheck="false" cols="40" rows="5" type="text" id="sql-json-input">\{\}</textarea>
                </div>
                <div class="sql-response" id="sql-response"></div>
            </div>
        `;

        const sqlInput = new SQLInput();
        sqlInput.render();

        const sqlResponse = new SQLResponse();
        sqlResponse.render();

        document.getElementById('sql-json-input').addEventListener('keydown', function(e) {
            if (e.key == 'Tab') {
              e.preventDefault();
              const start = this.selectionStart;
              const end = this.selectionEnd;
          
              // set textarea value to: text before caret + tab + text after caret
              this.value = this.value.substring(0, start) +
                "\t" + this.value.substring(end);
          
              // put caret at right position again
              this.selectionStart =
                this.selectionEnd = start + 1;
            }
          });
    }
}