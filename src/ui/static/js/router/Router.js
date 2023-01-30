import Component from "../lib/component.js";
import store from "../store/index.js";
import Navbar from "../views/layout/nav/Navbar.js";
import Sidebar from "../views/layout/sidebar/Sidebar.js";
import PlaygroundView from "../views/pages/pg/PlaygroundView.js";
import DatabaseView from "../views/pages/db/DatabaseView.js";
import AuthView from "../views/pages/auth/AuthView.js";
import SettingsView from "../views/pages/settings/SettingsView.js";
import InfoView from "../views/pages/info/InfoView.js";

export default class Router extends Component{
    constructor() {
        super({
            store,
            subscriptions: ["activePage"]
        });
    }
    async render() {

        const navbar = new Navbar();
        await navbar.render();
        
        const sidebar = new Sidebar();
        await sidebar.render();
        
        switch(store.state.activePage) {
            case "database":
                const databaseView = new DatabaseView();
                await databaseView.render();
                break;
            case "playground":
                const playgroundView = new PlaygroundView();
                await playgroundView.render();
                break;
            case "users":
                const authView = new AuthView();
                await authView.render();
                break;
            case "settings":
                const settingsView = new SettingsView();
                await settingsView.render();
                break;
            case "info":
                const infoView = new InfoView();
                await infoView.render();
                break;
            default:
                const defaultView = new DatabaseView();
                await defaultView.render();
        }
    }
}