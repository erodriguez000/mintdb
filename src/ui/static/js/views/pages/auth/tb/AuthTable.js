import Component from "../../../../lib/component.js";
import store from "../../../../store/index.js";

export default class AuthTable extends Component {
    constructor() {
        super({
            store,
            element: document.getElementById("auth-table"),
            subscriptions: []
        })
    }

    async render() {
        const data = [
            {id: 1, username: "eric@gmail.com", authorization: "admin", active: true, last_online: "-"}, 
            {id: 2, username: "lucy@gmail.com", authorization: "user", active: false, last_online: "-"},
        ];
        this.element.innerHTML = /*html*/`
        <tr>
            <th>id</th>
            <th>username</th>
            <th>active</th>
            <th>last online</th>
            <th>authorization</th>
        </tr>
        ${data.map(user => {
             return /*html*/`
                <tr>
                    <td>${user.id}</td>
                    <td>${user.username}</td>
                    <td>${user.active}</td>
                    <td>${user.last_online}</td>
                    <td>${user.authorization}</td>
                </tr>
            `;
        }).join('')}
        `;
    }
}