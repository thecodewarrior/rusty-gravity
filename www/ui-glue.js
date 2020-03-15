class UIGlue {
    static run(fn, ...args) {
        return window.app[fn](...args)
    }
    static runner(fn) {
        return (...args) => UIGlue.run(fn, ...args)
    }

    static hook_event_direct(target, type, fn) {
        target.addEventListener(type, UIGlue.runner(fn))
    }

    static hook_event(selector, type, fn) {
        document.querySelector(selector).addEventListener(type, UIGlue.runner(fn))
    }

    static set_interval(fn, delay) {
        return setInterval(UIGlue.runner(fn), delay)
    }
    static clear_interval(id) {
        return clearInterval(id)
    }

    static set_timeout(fn, delay) {
        return setTimeout(UIGlue.runner(fn), delay)
    }
    static clear_timeout(id) {
        return clearTimeout(id)
    }

    static request_animation_frame(fn) {
        requestAnimationFrame(UIGlue.runner(fn))
    }
}