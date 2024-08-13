# box aliases / functions for bash / zsh

# start default box by just running box, requires BOX_IMAGE env var!
function box() {
    if [[ "$#" -eq 0 ]]; then
        if [[ -n "$BOX_CONTAINER" ]] && box exists "$BOX_CONTAINER"; then
            echo "Box is already running"
            return
        fi

        unset BOX_CONTAINER
        export BOX_CONTAINER="$(box start)"
    else
        command box "$@"
    fi
}
