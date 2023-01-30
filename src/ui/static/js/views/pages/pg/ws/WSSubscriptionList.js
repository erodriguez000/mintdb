import Component from "../../../../lib/component.js";
import mintDB from "../../../../lib/mint.js";
import store from "../../../../store/index.js";

export default class WSSubscriptionList extends Component {
    constructor() {
        super({
            store,
            element: document.getElementById("sub-list"),
            subscriptions: ["subscriptions"]
        });
    }

    render() {
        this.element.innerHTML = /*html*/`
            <p>subscriptions:</p>
            ${store.state.subscriptions.map(sub => {
                return /*html*/ `
                    <div class="sub-badge" id="sub-${sub}">
                        <p>${sub}</p>
                        <svg xmlns="http://www.w3.org/2000/svg" class="sub-rm-icon" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                            <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                            <line x1="18" y1="6" x2="6" y2="18"></line>
                            <line x1="6" y1="6" x2="18" y2="18"></line>
                        </svg>
                    </div>
                `;
            }).join('')}
        `;
        const subBadgeEl = document.querySelectorAll(".sub-badge");
        subBadgeEl.forEach(el => el.addEventListener("click", () => {
                const sub = el.id.split("-");
                mintDB.removeSubscription(sub[1]);
                const payload = mintDB.subscriptions;
                store.dispatch('setSubscriptions', payload);
        }));
    }
}