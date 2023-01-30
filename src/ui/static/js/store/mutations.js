export default {
    setActivePage(state, payload) {
      state.activePage = payload.activePage;
    },
    setActiveTable(state, payload) {
      state.activeTable = payload;
    },
    setDocuments(state, payload) {
      state.tableDocs = payload;
    },
    setActiveDocument(state, payload) {
      state.activeDocument = payload.activeDocument;
    },
    toggleSocket(state, payload) {
      state.socketConnected = payload.socketConnected;
    },
    setSubscriptions(state, payload) {
      state.subscriptions = payload;
    },
    setWebsocketResponse(state, payload) {
      state.wsResponse = payload;
    },
    fetchQuery(state, payload) {
      state.sqlResponse = payload;
    },
    fetchTables(state, payload) {
      state.tables = payload;
    },
    deleteKeyValue(state, payload) {
      // TODO: remove key value correctly
    }
};