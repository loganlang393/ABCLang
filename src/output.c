#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

typedef struct MarkRefs MarkRefs;
typedef struct Reference Reference;
typedef struct Heap Heap;

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
	size_t total_heap_size;
	size_t current_heap_max;
	size_t current_allocated_size;
	Reference* entries;
	size_t total_num_entries;
};

Reference gc_allocate(struct Heap* h, size_t s, MarkRefs* mc) {
	if (h->current_allocated_size + s <= h->current_heap_max) {
		Reference entry = {h->bump_pointer, s, mc, true, true};
		h->current_allocated_size += s;
		h->bump_pointer += s;
		return entry;
	}
typedef struct test {
} test;

int x = /* unsupported expr */;
