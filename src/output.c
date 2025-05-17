#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

typedef struct MarkRefs MarkRefs;
typedef struct Reference Reference;
typedef struct Heap Heap;

struct MarkRefs {
	size_t object_size;
	bool allocated;
};

struct Reference {
	char* object_location;
	size_t object_size;
	MarkRefs* children;
	bool allocated;
	bool mark;
};

struct Heap {
	char* start;
	char* mid;
	char* bump_pointer;
	bool on_start;
	char* total_heap_size;
	char* current_heap_max;
	Reference* entries;
	size_t total_num_entries;
	size_t num_entries;
};

void gc_reallocate(struct Heap* h) {
	if(h->on_start) {
		h->on_start = false;
		h->bump_pointer = h->mid;
	}else {
		h->on_start = true;
		h->bump_pointer = h->start;
	}
	Reference* current_list = h->entries;
	h->entries = malloc(sizeof(Reference) * h->total_num_entries);
	for(size_t i = 0; i < h->total_num_entries; i++) {
		if(current_list[i]->mark) {
			h->entries[h->num_entries] = *gc_allocate(h, current_list[i]->object_size, current_list[i]->children);
			free(current_list[i]);
			current_list[i] = NULL
		}else {
			free(current_list[i]);
			current_list[i] = NULL;
		}
	}
	free(current_list);
	current_list = NULL
}

Reference gc_allocate(struct Heap* h, size_t s, MarkRefs* mc) {
	if(h->on_start) {
		if (h->bump_pointer + s <= h->mid) {
			if (h->num_entries == h->total_num_entries) {
				gc_reallocate(&h);
				if (h->num_entries == h->total_num_entries) {
					Refernce entry = {h->bump_pointer, s, mc, false, false};
					return entry;
				} else {
					return gc_allocate(&h, s, &mc);
				}
			}
			Reference entry = {h->bump_pointer, s, mc, true, true};
			h->entries[num_entries] = entry;
			h->num_entries++;
			h->bump_pointer += s;
			return entry;
		} else {
			gc_reallocate(&h);
			return gc_allocate(&h, s, &mc);
		}
	} else {
		if (h->bump_pointer + s <= h->total_heap_size) {
			if (h->num_entries == h->total_num_entries) {
				gc_reallocate(&h);
				if (h->num_entries == h-> total_num_entries) {
					Reference entry = {h->bump_pointer, s, mc, false, false};
					return NULL;
				} else {
					return gc_allocate(&h, s, &mc);
				}
			}
			Reference entry = {h->bump_pointer, s, mc, true, true};
			h->num_entries++;
			h->bump_pointer += s;
			return &entry;
		} else {
			gc_reallocate(&h);
			return gc_allocate(&h, s, &mc);
		}
	}
}
void gc_deallocate(struct Heap* h, Reference reference) {
	h->current_allocated_size -= reference->object_size;
	reference->mark=false;
}
