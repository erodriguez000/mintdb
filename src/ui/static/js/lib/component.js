import Store from "../store/store.js";

export default class Component {
    constructor(props = {}) {
        let self = this;

        this.render = this.render || function () {};

        if(props.store instanceof Store) {
            //props.store.events.subscribe('stateChange', async() => self.render());
            if(props.hasOwnProperty("subscriptions") && props.subscriptions instanceof Array) {
                props.subscriptions.forEach(subKey => props.store.events.subscribe(subKey, async () => self.render()));
            }
        }
        
        if(props.hasOwnProperty("element")) {
            this.element = props.element;
        }
    }
}