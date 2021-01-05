#pragma GCC optimize("O3","unroll-loops","omit-frame-pointer","inline")

#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <stdbool.h>
#include <inttypes.h>


/*
* GAME PLAN
* Idea:
*   initialize all tome spell plus
*   pre-calculate couple of fastest ways to brew all 720 potions
*       - some paths with "rest"
*       - check most viable spells based on result (most used)
*       - always learn those X spells
*/

/*
Current:
bfs if can brew any in 7steps

Ideas:
only top3 else learn or cast for balance!
*/

#define BLUE    2
#define GREEN   5
#define ORANGE  7
#define GOLD    9

/*
* HARD LIMITS and settings
*/

#define MAX_SPELLS 14

/*
* DEBUG
*/

#ifndef GAME_DBG
# define GAME_DBG 1
#endif
#define LOG(format, ...) if (GAME_DBG) fprintf(stderr, "%d: " format "\n", __LINE__, ##__VA_ARGS__)
#define ABS(x) x < 0 ? -x : x

typedef struct s_inventory{
    int     resources[4];
    int     res_count;
    int     score;
}               t_inv;

typedef struct  s_potion{
    int     resources[4];
    int     id;
    int     price;
}               t_potion;

typedef struct s_spell{
    int         resources[4];
    int         id;
    bool        castable;
    bool        repeatable;
}               t_spell;

typedef struct s_tome{
    t_spell     spells[6];
    int         tax[6];
    int         n_spells;
}               t_tome;

typedef struct  s_spellbook{
    t_spell     spells[46];
    uint8_t     n_spells;
}               t_spellbook;
/*
*   Utility Functions
*/

int     sum_resources(const int res[4])
{
    return (res[0] + res[1] + res[2] + res[3]);
}

void resources_add(const int res_a[4],const int res_b[4], int result[4])
{
	for (int i = 0; i < 4; i++)
		result[i] = res_a[i] + res_b[i];
}

bool negative_resource(const int res[4])
{
    for (int i = 0; i < 4; i++)
    {
        if (res[i] < 0)
            return (true);
    }
    return (false);
}

bool    can_action(const int inv[4], const int action[4])
{
    int after[4];
    resources_add(inv, action, after);
    return (!negative_resource(after) && sum_resources(after) <= 10);
}

void    update_potion(t_potion potions[5],\
                    const int action_id,\
                    const int i0,\
                    const int i1,\
                    const int i2,\
                    const int i3,\
                    const int price)
{
    for (int i = 0; i < 5; i++)
    {
        if (potions[i].id == 0)
        {
            potions[i].id = action_id;
            potions[i].resources[0] = i0;
            potions[i].resources[1] = i1;
            potions[i].resources[2] = i2;
            potions[i].resources[3] = i3;
            potions[i].price = price;
            break ;
        }
    }
}

void    update_spellbook(t_spellbook *spellbook,\
                    const int action_id,\
                    const int i0,\
                    const int i1,\
                    const int i2,\
                    const int i3,\
                    const bool castable,\
                    const bool repeatable)
{
    for (int i = 0; i < MAX_SPELLS; i++)
    {
        if (spellbook->spells[i].id == 0)
        {
            spellbook->spells[i].id = action_id;
            spellbook->spells[i].resources[0] = i0;
            spellbook->spells[i].resources[1] = i1;
            spellbook->spells[i].resources[2] = i2;
            spellbook->spells[i].resources[3] = i3;
            spellbook->spells[i].castable = castable;
            spellbook->spells[i].repeatable = repeatable;
            spellbook->n_spells += 1;
            break;
        }
    }
}

void    update_tome(t_tome  *tome,
                    const int action_id,
                    const int i0,
                    const int i1,
                    const int i2,
                    const int i3,
                    const bool repeatable,
                    const int tax,
                    const int tome_index)
{
    tome->spells[tome_index].id = action_id;
    tome->spells[tome_index].resources[0] = i0;
    tome->spells[tome_index].resources[1] = i1;
    tome->spells[tome_index].resources[2] = i2;
    tome->spells[tome_index].resources[3] = i3;
    tome->spells[tome_index].repeatable = repeatable;
    tome->spells[tome_index].castable = false;
    tome->tax[tome_index] = tax;
    tome->n_spells += 1;
}

int     can_brew(const t_potion potions[5], const t_inv inv)
{
    int ret = -1;
    int best = -1;
    for (int i = 0; i < 5; i++)
    {
        if (can_action(inv.resources, potions[i].resources) && potions[i].price > best)
        {
            best = potions[i].price;
            ret = potions[i].id;
        }
    }
    return (ret);
}

int     select_learn(const t_tome tome, const t_inv inv)
{
    int     ret = -1;
    int     val = 0;
    int     cost_count = 0;

    if (tome.n_spells)
    {
        if (tome.tax[0] > 2 && inv.res_count < 8)
            return (tome.spells[0].id);
        for (int t=0; t < tome.n_spells; t++)
        {
            val = tome.spells[t].resources[0] * BLUE + tome.spells[t].resources[1] * GREEN + tome.spells[t].resources[2] * ORANGE + tome.spells[t].resources[3] * GOLD;
            cost_count = 0;
            for (int c=0; c < 4; c++)
                cost_count -= tome.spells[t].resources[c] < 0 ? tome.spells[t].resources[c] : 0;
            if (cost_count < 4 && val > 3 + t && inv.resources[0] >= t)
            {
                LOG("LEARN VALUE = %d", val);
                return (tome.spells[t].id);
            }
        }
    }
    return (ret);
}

#define MAX_DEPTH 20

typedef	struct s_queue
{
	int				path[MAX_DEPTH];
	int				len;
	int				resources[4];
	struct s_queue	*next;
}				t_q;

t_q		*q_append(t_q *tail, int path[MAX_DEPTH], int len, int resources[4])
{
	t_q		*ret = NULL;

	ret = (t_q *)malloc(sizeof(t_q));
	if (!ret)
		return (NULL);
	memcpy(ret->path, path, sizeof(int) * MAX_DEPTH);
	ret->len = len;
	memcpy(ret->resources, resources, sizeof(int) * 4);
	ret->next = NULL;
	if (tail)
		tail->next = ret;
	return (ret);
}

t_q		*q_pop(t_q **head, t_q**tail)
{
	t_q	*ret;
	ret = *head;
	*head = ret->next;
	if (*head == NULL)
		*tail = NULL;
	return (ret);
}

t_q		*create_queue(const int inv[4], const t_spellbook *spellbook, t_q **ret_tail)
{
	t_q *head = NULL;
	t_q *tail = NULL;
	int	after[4];
	int path[MAX_DEPTH];

	memset(path, 0, sizeof(int) * MAX_DEPTH);

	for (int i = 0; i < spellbook->n_spells; i++)
	{
		if (spellbook->spells[i].castable && can_action(inv, spellbook->spells[i].resources))
		{
			resources_add(inv, spellbook->spells[i].resources, after);
			path[0] = spellbook->spells[i].id;
			if (tail != NULL)
				tail = q_append(tail, path, 1, after);
			else
			{
				head = q_append(tail, path, 1, after);
				tail = head;
			}
		}
	}
	*ret_tail = tail;
	return (head);
}

bool	not_visited(const int path[7], const int len, const int id)
{
	for (int i=0; i < len; i++)
		if (path[i] == id)
			return (false);
	return (true);
}

int		bfs(const int start[4], const t_potion *potions, const t_spellbook *spellbook, int path[MAX_DEPTH])
{
	t_q		*q;
	t_q		*q_tail;
	t_q		*current;
	int		after[4];
    int     extra_step = 0;
    int     visited[11][11][11][11][2];

    memset(path, 0, sizeof(int) * MAX_DEPTH);
    memset(visited, 0, sizeof(int) * 11 * 11 * 11 * 11 * 2);
	q = create_queue(start, spellbook, &q_tail);
	while (q)
	{
		current = q_pop(&q, &q_tail);
        for (int p = 0; p < 5; p++) {
		    if (can_action(current->resources, potions[p].resources))
            {
                memcpy(path, current->path, sizeof(int) * MAX_DEPTH);
                return current->len;
            }
        }
		if (current->len < MAX_DEPTH)
		{
			for (int i = 0; i < spellbook->n_spells; i++)
			{
				if (spellbook->spells[i].castable && can_action(current->resources, spellbook->spells[i].resources))
				{
					resources_add(current->resources, spellbook->spells[i].resources, after);
                    if (!visited[after[0]][after[1]][after[2]][after[3]][0])
                    {
					    current->path[current->len] = spellbook->spells[i].id;
                        visited[after[0]][after[1]][after[2]][after[3]][0] = 1;
                    }
                    else
                    {
                        current->path[current->len] = -1;
                        memcpy(after, current->resources, sizeof(int) * 4);
                    }
					if (q_tail != NULL)
						q_tail = q_append(q_tail, current->path, current->len + 1, after);
					else
					{
						q = q_append(q_tail, current->path, current->len + 1, after);
						q_tail = q;
					}
				}
			} 
		}
		free(current);
	}
	return (-1);
}

int     select_spell_low_res(const t_spellbook *spellbook, const int inv[4])
{
    int best = inv[0] * BLUE + inv[1] * GREEN + inv[2] * ORANGE + inv[3] * GOLD;
    int best_id = -1;
    int current = 0;
    for (int i = 0; i < spellbook->n_spells; i++)
    {
        if (spellbook->spells[i].castable && !negative_resource(spellbook->spells[i].resources))
        {
            current = spellbook->spells[i].resources[0] * BLUE + spellbook->spells[i].resources[1] * GREEN + spellbook->spells[i].resources[2] * ORANGE + spellbook->spells[i].resources[3] * GOLD;
            if (current > best)
            {
                best = current;
                best_id = spellbook->spells[i].id;
            }
        }
    }
    return (best_id);
}

void    sort_potions(t_potion   potions[5], int sorted_pots[5])
{
    int c;
    memset(sorted_pots, -1, sizeof(int) * 5);
    for (int i=0; i < 5; i++)
    {
        c = 0;
        for(int j=0; j < 5; j++)
            if (potions[i].id != potions[j].id && potions[j].price / sum_resources(potions[j].resources) > potions[i].price / sum_resources(potions[i].resources))
            // if (potions[i].id != potions[j].id && potions[j].price > potions[i].price)
                c++;
        while(sorted_pots[c] != -1 && c < 5)
            c++;
        sorted_pots[c] = i;
    }
    fprintf(stderr, "SORTED POTS: ");
    for (int i = 0; i < 5; i++)
        fprintf(stderr, "%d ", potions[sorted_pots[i]].id);
    fprintf(stderr, "\n");
}

int     best_of_three(int options[3])
{
    int ret;
    options[0] = options[0] < 1 ? MAX_DEPTH + 1 : options[0];
    options[1] = options[1] < 1 ? MAX_DEPTH + 1 : options[1];
    options[2] = options[2] < 1 ? MAX_DEPTH + 1 : options[2];
    if (options[0] < options[1])
        ret = options[0] < options[2] ? 0 : 2;
    else
        ret = options[1] < options[2] ? 1 : 2;
    return (options[ret] <= MAX_DEPTH ? ret : -1);
}

int main()
{
    t_inv       my_inventory;
    t_inv       enemy_inventory;
    t_potion    potions[5];
    int         sp[5];
    t_tome      tome;
    t_spellbook spellbook;
    int         ret = 0;
    int         ret_bfs[3];
    int         paths_bfs[3][MAX_DEPTH];
    int         path[MAX_DEPTH];
    bool        last_rest = true;
    // counters
    int n_pot = 0;
    int n_tome = 0;
    
    memset(&spellbook, 0, sizeof(t_spellbook));
    memset(&tome, 0, sizeof(t_tome));
    memset(potions, 0, sizeof(t_potion) * 5);
    memset(path, 0, sizeof(int) * MAX_DEPTH);

    // game loop
    while (1) {
        // the number of spells and recipes in play
        int action_count;
        scanf("%d", &action_count);
        for (int i = 0; i < action_count; i++) {
            // the unique ID of this spell or recipe
            int action_id;
            // in the first league: BREW; later: CAST, OPPONENT_CAST, LEARN, BREW
            char action_type[21];
            // tier-0 ingredient change
            int delta_0;
            // tier-1 ingredient change
            int delta_1;
            // tier-2 ingredient change
            int delta_2;
            // tier-3 ingredient change
            int delta_3;
            // the price in rupees if this is a potion
            int price;
            // in the first two leagues: always 0; later: the index in the tome if this is a tome spell, equal to the read-ahead tax
            int tome_index;
            // in the first two leagues: always 0; later: the amount of taxed tier-0 ingredients you gain from learning this spell
            int tax_count;
            // in the first league: always 0; later: 1 if this is a castable player spell
            bool castable;
            // for the first two leagues: always 0; later: 1 if this is a repeatable player spell
            bool repeatable;
            int _castable;
            int _repeatable;
            scanf("%d%s%d%d%d%d%d%d%d%d%d", &action_id, action_type, &delta_0, &delta_1, &delta_2, &delta_3, &price, &tome_index, &tax_count, &_castable, &_repeatable);
            castable = _castable;
            repeatable = _repeatable;
            // Potions
            if (!strcmp(action_type, "BREW"))
                update_potion(potions, action_id, delta_0, delta_1, delta_2, delta_3, price);
            // SpellBook
            if (!strcmp(action_type, "CAST"))
                update_spellbook(&spellbook, action_id, delta_0, delta_1, delta_2, delta_3, castable, repeatable);
            // Tome
            if (!strcmp(action_type, "LEARN"))
                update_tome(&tome, action_id, delta_0, delta_1, delta_2, delta_3, repeatable, tax_count, tome_index);

        }
        scanf("%d%d%d%d%d", &my_inventory.resources[0], &my_inventory.resources[1], &my_inventory.resources[2], &my_inventory.resources[3], &my_inventory.score);
        my_inventory.res_count = my_inventory.resources[0] + my_inventory.resources[1] + my_inventory.resources[2] + my_inventory.resources[3];
        scanf("%d%d%d%d%d", &enemy_inventory.resources[0], &enemy_inventory.resources[1], &enemy_inventory.resources[2], &enemy_inventory.resources[3], &enemy_inventory.score);
        enemy_inventory.res_count = enemy_inventory.resources[0] + enemy_inventory.resources[1] + enemy_inventory.resources[2] + enemy_inventory.resources[3];
        // Write an action using printf(). DON'T FORGET THE TRAILING \n
        // To debug: fprintf(stderr, "Debug messages...\n");
        sort_potions(potions, sp);
        if ((ret = can_brew(potions, my_inventory)) != -1)
        {
            printf("BREW %d\n", ret);
            goto reset;
        }
        if ((spellbook.n_spells < MAX_SPELLS) && (ret = select_learn(tome, my_inventory)) != -1)
        {
            printf("LEARN %d\n", ret);
            goto reset;
        }
        if (my_inventory.res_count < 4 && (ret = select_spell_low_res(&spellbook, my_inventory.resources)) != -1)
        {
            last_rest = false;
            printf("CAST %d\n", ret);
            goto reset;
        }
        for (int i=0; i < 2; i++)
        {
            ret = bfs(my_inventory.resources, potions, &spellbook, path);
        // ret_bfs[1] = bfs(my_inventory.resources, potions[sp[1]].resources, &spellbook, paths_bfs[1]);
        // ret_bfs[2] = bfs(my_inventory.resources, potions[sp[2]].resources, &spellbook, paths_bfs[2]);
        // ret = best_of_three(ret_bfs);
            LOG("BFS%d done %d", i, ret);
            if (ret != -1)
            {
                if (path[0] != -1)
                {
                    last_rest = false;
                    printf("CAST %d\n", path[0]);
                }
                else
                {
                    last_rest = true;
                    printf("REST PATH\n");
                }
                goto reset;
            }
        }
        if (last_rest && (ret = select_learn(tome, my_inventory)) != -1)
        {
            printf("LEARN %d\n", ret);
            goto reset;
        }
        last_rest = true;
        printf("REST\n");
        //TEST PRINTS
        // for (int i=0; i < 5; i++)
        //     LOG("P-%d", potions[i].id);
        // for (int i=0; i < spellbook.n_spells; i++)
        //     LOG("S-%d", spellbook.spells[i].id);
        // for (int i=0; i < tome.n_spells; i++)
        //     LOG("T-%d", tome.spells[i].id);
        // in the first league: BREW <id> | WAIT; later: BREW <id> | CAST <id> [<times>] | LEARN <id> | REST | WAIT
        
        // RESET
reset:
        fflush(stdout);
        memset(potions, 0, sizeof(t_potion) * 5);
        memset(&spellbook, 0, sizeof(t_spellbook));
        memset(&tome, 0, sizeof(t_tome));
        memset(path, 0, sizeof(int) * MAX_DEPTH);
        // memset(path, 0, sizeof(int) * MAX_DEPTH * 3);
        ret = 0;
    }

    return (0);
}