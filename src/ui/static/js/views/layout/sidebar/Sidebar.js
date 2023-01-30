import Component from "../../../lib/component.js";
import store from "../../../store/index.js"
export default class Sidebar extends Component {
    constructor() {
        super({
            store,
            element: document.getElementById("sidebar"),
            subscriptions: ["activePage"]
        })
    }
    async render() {
        this.element.innerHTML = /*html*/`
        <div class="sidebar-middle">
            <ul class="sidebar-list">
                <li class="sidebar-list-item" id="database">
                    <div class="sidebar-link">
                        <svg xmlns="http://www.w3.org/2000/svg" class="sidebar-icon" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                            <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                            <ellipse cx="12" cy="6" rx="8" ry="3"></ellipse>
                            <path d="M4 6v6a8 3 0 0 0 16 0v-6"></path>
                            <path d="M4 12v6a8 3 0 0 0 16 0v-6"></path>
                        </svg>
                        <div class="hidden-sidebar">Dashboard</div>
                    </div>
                </li>
                <li class="sidebar-list-item" id="playground">
                    <div class="sidebar-link">
                        <svg xmlns="http://www.w3.org/2000/svg" class="sidebar-icon" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                            <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                            <circle cx="12" cy="5" r="2"></circle>
                            <path d="M20 12.5v4.75a0.734 .734 0 0 1 -.055 .325a0.704 .704 0 0 1 -.348 .366l-5.462 2.58a4.998 4.998 0 0 1 -4.27 0l-5.462 -2.58a0.705 .705 0 0 1 -.401 -.691l-.002 -4.75"></path>
                            <path d="M4.431 12.216l5.634 -2.332a5.065 5.065 0 0 1 3.87 0l5.634 2.332a0.692 .692 0 0 1 .028 1.269l-5.462 2.543a5.064 5.064 0 0 1 -4.27 0l-5.462 -2.543a0.691 .691 0 0 1 .028 -1.27z"></path>
                            <line x1="12" y1="7" x2="12" y2="13"></line>
                        </svg>
                        <div class="hidden-sidebar">Playground</div>
                    </div>
                </li>
                <li class="sidebar-list-item" id="users">
                    <div class="sidebar-link">
                        <svg xmlns="http://www.w3.org/2000/svg" class="sidebar-icon" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                            <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                            <circle cx="12" cy="7" r="4"></circle>
                            <path d="M6 21v-2a4 4 0 0 1 4 -4h4a4 4 0 0 1 4 4v2"></path>
                        </svg>
                        <div class="hidden-sidebar">Users</div>
                    </div>
                </li>
            </ul>
        </div>
        <div class="sidebar-bottom">
            <ul class="sidebar-list">
                <li class="sidebar-list-item" id="settings">
                    <div class="sidebar-link">
                        <svg xmlns="http://www.w3.org/2000/svg" class="sidebar-icon" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                            <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                            <circle cx="14" cy="6" r="2"></circle>
                            <line x1="4" y1="6" x2="12" y2="6"></line>
                            <line x1="16" y1="6" x2="20" y2="6"></line>
                            <circle cx="8" cy="12" r="2"></circle>
                            <line x1="4" y1="12" x2="6" y2="12"></line>
                            <line x1="10" y1="12" x2="20" y2="12"></line>
                            <circle cx="17" cy="18" r="2"></circle>
                            <line x1="4" y1="18" x2="15" y2="18"></line>
                            <line x1="19" y1="18" x2="20" y2="18"></line>
                        </svg>
                        <div class="hidden-sidebar">Settings</div>
                    </div>
                </li>
                <li class="sidebar-list-item" id="info">
                    <div class="sidebar-link">
                        <svg xmlns="http://www.w3.org/2000/svg" class="sidebar-icon" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                            <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                            <circle cx="12" cy="12" r="9"></circle>
                            <line x1="12" y1="8" x2="12.01" y2="8"></line>
                            <polyline points="11 12 12 12 12 16 13 16"></polyline>
                        </svg>
                        <div class="hidden-sidebar">Info</div>
                    </div>
                </li>
            </ul>
        </div>
        `;
        this.element.querySelector(`#${store.state.activePage}`).classList.add("active");
        const sidebarLinks = this.element.querySelectorAll(".sidebar-list-item");
        sidebarLinks.forEach(link => link.onclick = () => { store.dispatch("setActivePage", {activePage: link.id})});
    }
}