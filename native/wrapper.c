#include "wrapper.h"

extern gvplugin_library_t gvplugin_dot_layout_LTX_library;

Agnode_t *rust_aghead(Agedge_t * ptr) {
    return aghead(ptr);
}
Agnode_t *rust_agtail(Agedge_t * ptr) {
    return agtail(ptr);
}