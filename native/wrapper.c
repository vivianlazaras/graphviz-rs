#include "wrapper.h"

Agnode_t *rust_aghead(Agedge_t * ptr) {
    return aghead(ptr);
}
Agnode_t *rust_agtail(Agedge_t * ptr) {
    return agtail(ptr);
}