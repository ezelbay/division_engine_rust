#pragma once

#define DIVISION_LIST_DEFINE(type) \
typedef struct { type* items; size_t length; size_t capacity; } List_##type;

#define DIVISION_LIST_CREATE(item_type, capacity) \
    (List_##item_type) { malloc(sizeof(item_type) * capacity), 0, capacity };

#define DIVISION_LIST_APPEND(list, item) \
    list.length++; \
    if (list.length > list.capacity) {   \
        int new_capacity = list.capacity * 2; \
        list.items = realloc(list.items, sizeof(item) * new_capacity); \
    }                                                                 \
    list.items[list.length - 1] = item;
