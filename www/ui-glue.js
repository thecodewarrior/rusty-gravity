class UIGlue {
    static hook_event(target, type, functionName) {
        target.addEventListener(type, (event) => {
            window.app[functionName](window.app, event)
        })
    }
}