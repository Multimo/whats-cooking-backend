RECIPES:
GET:
{
    id: i32',
    ingredents: ingredients[], 
    name: string // name eg. pizz
    decription: string // instructions or description of the recipe
    url: string // link to image
    link: string // link to website if there
}[]

ingredients:
{
    id: IngredientId, 
    quantity: number, 
    metric: string
},


ingredient:
{
    id: string
    names: {
        english: string
        fr: string
        nl: string
    },
    group: 'herbs and spices' | 'meat'
    subgroup: 'herbs'
    url?: string
}

shopping list:
{
    recipes: Recipes[]
    id: string
    date: string
    list: listItem[]
}

listItem {
    complete: bool
    ingredientId: i32
    name: string
    quantiy: number
    metric: string // 'ammount' | 'handfuls' | 'cups'
}