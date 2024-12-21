local entity_a = world:spawn()
local entity_b = world:spawn()
local entity_c = world:spawn()
local entity_d = _get_entity_with_test_component("CompWithFromWorldAndComponentData")

local component_with = world:get_type_by_name("CompWithFromWorldAndComponentData")
local component_without = world:get_type_by_name("CompWithDefaultAndComponentData")

world:add_default_component(entity_a, component_with)
world:add_default_component(entity_b, component_with)
world:add_default_component(entity_c, component_with)

world:add_default_component(entity_b, component_without)

local found_entities = {}
for entity, comp in pairs(world:query({component_with}):with(component_with):without(component_without):build()) do
    table.insert(found_entities, entity)
end

assert(#found_entities == 3, "Expected 3 entities, got " .. #found_entities)

expected_entities = {
    entity_d,
    entity_a,
    entity_c,
}

for i, entity in ipairs(found_entities) do
    assert(entity:index() == expected_entities[i]:index(), "Expected entity " .. expected_entities[i]:index() .. " but got " .. entity:index())
end

