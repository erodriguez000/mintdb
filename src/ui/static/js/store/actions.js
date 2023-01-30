export default {
    setActivePage(ctx, payload){
      ctx.commit('setActivePage', payload);
    },
    setActiveTable(ctx, payload) {
      ctx.commit('setActiveTable', payload);
    },
    setDocuments(ctx, payload) {
      ctx.commit('setDocuments', payload);
    },
    setActiveDocument(ctx, payload) {
      ctx.commit('setActiveDocument', payload);
    },
    toggleSocket(ctx, payload) {
      ctx.commit('toggleSocket', payload);
    },
    setSubscriptions(ctx, payload) {
      ctx.commit('setSubscriptions', payload);
    },
    setWebsocketResponse(ctx, payload) {
      ctx.commit('setWebsocketResponse', payload);
    },
    fetchQuery(ctx, payload) {
      ctx.commit('fetchQuery', payload);
    },
    fetchTables(ctx, payload) {
      ctx.commit('fetchTables', payload);
    },
    deleteKeyValue(ctx, payload) {
      ctx.commit('deleteKeyValue', payload);
    }
  };