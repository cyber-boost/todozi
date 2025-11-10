// Error handling
enum TodoziErrorType {
    ValidationError = 'ValidationError',
    NotFoundError = 'NotFoundError'
}

class TodoziError extends Error {
    constructor(
        public type: TodoziErrorType,
        message: string
    ) {
        super(message);
        this.name = 'TodoziError';
    }
}

// Branded types for ID safety
type TagID = string & { readonly __brand: unique symbol };
type Category = string & { readonly __brand: unique symbol };

// Helper to create branded IDs
const createTagID = (id: string): TagID => id as TagID;
const createCategory = (category: string): Category => category as Category;

// Tag interface
interface Tag {
    readonly id: TagID;
    name: string;
    description?: string;
    color?: string;
    category?: Category;
    usage_count: number;
    readonly created_at: Date;
    updated_at: Date;
}

// TagUpdate builder
interface TagUpdate {
    name?: string;
    description?: string;
    color?: string;
    category?: string;
}

class TagUpdateBuilder {
    private update: TagUpdate = {};

    static new(): TagUpdateBuilder {
        return new TagUpdateBuilder();
    }

    withName(name: string): this {
        this.update.name = name;
        return this;
    }

    withDescription(description: string): this {
        this.update.description = description;
        return this;
    }

    withColor(color: string): this {
        this.update.color = color;
        return this;
    }

    withCategory(category: string): this {
        this.update.category = category;
        return this;
    }

    build(): Readonly<TagUpdate> {
        return Object.freeze({ ...this.update });
    }
}

// TagStatistics
interface TagStatistics {
    readonly total_tags: number;
    readonly total_categories: number;
    readonly total_relationships: number;
    readonly average_usage: number;
}

class TagStatisticsImpl implements TagStatistics {
    constructor(
        public readonly total_tags: number,
        public readonly total_categories: number,
        public readonly total_relationships: number,
        public readonly average_usage: number
    ) {}

    relationships_per_tag(): number {
        if (this.total_tags === 0) {
            return 0.0;
        }
        return this.total_relationships / this.total_tags;
    }
}

// Search options
interface SearchOptions {
    query?: string;
    category?: string;
    minUsage?: number;
    maxUsage?: number;
    sortBy?: TagSortBy;
    limit?: number;
}

// TagSortBy enum
enum TagSortBy {
    Name = 'Name',
    Usage = 'Usage',
    Created = 'Created',
    Updated = 'Updated'
}

// TagSearchQuery
interface TagSearchQuery {
    name_contains?: string;
    description_contains?: string;
    category?: string;
    color?: string;
    min_usage?: number;
    max_usage?: number;
    sort_by: TagSortBy;
    limit?: number;
}

class TagSearchQueryImpl implements TagSearchQuery {
    constructor(
        public name_contains?: string,
        public description_contains?: string,
        public category?: string,
        public color?: string,
        public min_usage?: number,
        public max_usage?: number,
        public sort_by: TagSortBy = TagSortBy.Name,
        public limit?: number
    ) {}

    static default(): TagSearchQuery {
        return new TagSearchQueryImpl();
    }
}

// TagManager class
class TagManager {
    private tags: Map<TagID, Tag> = new Map();
    private tagRelationships: Map<TagID, TagID[]> = new Map();
    private categoryTags: Map<Category, TagID[]> = new Map();

    async createTag(tagData: Omit<Tag, 'id' | 'created_at' | 'updated_at'>): Promise<TagID> {
        // Validation
        if (!tagData.name || tagData.name.trim().length === 0) {
            throw new TodoziError(TodoziErrorType.ValidationError, 'Tag name cannot be empty');
        }
        
        if (this.getTagByName(tagData.name)) {
            throw new TodoziError(TodoziErrorType.ValidationError, `Tag with name "${tagData.name}" already exists`);
        }

        const now = new Date();
        const id = createTagID(this.generateUUID());
        
        const tag: Tag = {
            ...tagData,
            id,
            created_at: now,
            updated_at: now,
            category: tagData.category ? createCategory(tagData.category) : undefined
        };

        if (tag.category) {
            this.categoryTags.set(tag.category, [
                ...(this.categoryTags.get(tag.category) || []),
                id
            ]);
        }

        this.tags.set(id, tag);
        return id;
    }

    getTag(tagId: TagID): Tag | undefined {
        return this.tags.get(tagId);
    }

    getTagByName(name: string): Tag | undefined {
        return Array.from(this.tags.values()).find(tag => tag.name === name);
    }

    getAllTags(): Tag[] {
        return Array.from(this.tags.values());
    }

    async updateTag(tagId: TagID, updates: TagUpdate): Promise<void> {
        const tag = this.tags.get(tagId);
        if (!tag) {
            throw new TodoziError(
                TodoziErrorType.NotFoundError,
                `Tag with ID "${tagId}" not found`
            );
        }

        const oldCategory = tag.category;

        if (updates.name !== undefined) {
            if (updates.name.trim().length === 0) {
                throw new TodoziError(TodoziErrorType.ValidationError, 'Tag name cannot be empty');
            }
            tag.name = updates.name;
        }

        if (updates.description !== undefined) {
            tag.description = updates.description;
        }

        if (updates.color !== undefined) {
            tag.color = updates.color;
        }

        if (updates.category !== undefined) {
            tag.category = updates.category ? createCategory(updates.category) : undefined;
            
            // Remove from old category
            if (oldCategory) {
                const categoryTags = this.categoryTags.get(oldCategory);
                if (categoryTags) {
                    this.categoryTags.set(oldCategory, categoryTags.filter(id => id !== tagId));
                }
            }
            
            // Add to new category
            if (tag.category) {
                this.categoryTags.set(tag.category, [
                    ...(this.categoryTags.get(tag.category) || []),
                    tagId
                ]);
            }
        }

        tag.updated_at = new Date();
    }

    async deleteTag(tagId: TagID): Promise<void> {
        const tag = this.tags.get(tagId);
        if (!tag) {
            throw new TodoziError(
                TodoziErrorType.NotFoundError,
                `Tag with ID "${tagId}" not found`
            );
        }

        this.tags.delete(tagId);

        // Remove from category
        if (tag.category) {
            const categoryTags = this.categoryTags.get(tag.category);
            if (categoryTags) {
                this.categoryTags.set(tag.category, categoryTags.filter(id => id !== tagId));
            }
        }

        // Remove relationships
        this.tagRelationships.delete(tagId);
        
        // Remove from other tags' relationships
        for (const [tagId, relationships] of this.tagRelationships.entries()) {
            this.tagRelationships.set(tagId, relationships.filter(id => id !== tagId));
        }
    }

    async addTagRelationship(tagId: TagID, relatedTagId: TagID): Promise<void> {
        if (!this.tags.has(tagId)) {
            throw new TodoziError(
                TodoziErrorType.NotFoundError,
                `Tag with ID "${tagId}" not found`
            );
        }
        
        if (!this.tags.has(relatedTagId)) {
            throw new TodoziError(
                TodoziErrorType.NotFoundError,
                `Related tag with ID "${relatedTagId}" not found`
            );
        }

        const relationships = this.tagRelationships.get(tagId) || [];
        if (!relationships.includes(relatedTagId)) {
            relationships.push(relatedTagId);
        }
        this.tagRelationships.set(tagId, relationships);
    }

    getRelatedTags(tagId: TagID): Tag[] {
        const relatedIds = this.tagRelationships.get(tagId);
        if (!relatedIds) return [];
        
        return relatedIds
            .map(id => this.tags.get(id))
            .filter((tag): tag is Tag => tag !== undefined);
    }

    searchTags(query: string): Tag[] {
        const queryLower = query.toLowerCase();
        return Array.from(this.tags.values()).filter(tag => 
            tag.name.toLowerCase().includes(queryLower) ||
            (tag.description && tag.description.toLowerCase().includes(queryLower))
        );
    }

    getTagsByCategory(category: string): Tag[] {
        const categoryKey = createCategory(category);
        const tagIds = this.categoryTags.get(categoryKey);
        if (!tagIds) return [];
        
        return tagIds
            .map(id => this.tags.get(id))
            .filter((tag): tag is Tag => tag !== undefined);
    }

    getAllCategories(): string[] {
        return Array.from(this.categoryTags.keys());
    }

    async incrementTagUsage(tagName: string): Promise<void> {
        const tag = this.getTagByName(tagName);
        if (tag) {
            tag.usage_count += 1;
            tag.updated_at = new Date();
        }
    }

    getMostUsedTags(limit: number): Tag[] {
        return Array.from(this.tags.values())
            .sort((a, b) => b.usage_count - a.usage_count)
            .slice(0, limit);
    }

    getRecentTags(limit: number): Tag[] {
        return Array.from(this.tags.values())
            .sort((a, b) => b.created_at.getTime() - a.created_at.getTime())
            .slice(0, limit);
    }

    getTagStatistics(): TagStatistics {
        const totalTags = this.tags.size;
        const totalCategories = this.categoryTags.size;
        const totalRelationships = Array.from(this.tagRelationships.values())
            .reduce((sum, relationships) => sum + relationships.length, 0);
        
        const averageUsage = totalTags > 0
            ? Array.from(this.tags.values()).reduce((sum, tag) => sum + tag.usage_count, 0) / totalTags
            : 0;

        return new TagStatisticsImpl(totalTags, totalCategories, totalRelationships, averageUsage);
    }

    async bulkCreateTags(tagNames: string[], category?: string): Promise<TagID[]> {
        const createdIds: TagID[] = [];
        
        for (const name of tagNames) {
            const tag: Omit<Tag, 'id' | 'created_at' | 'updated_at'> = {
                name,
                description: undefined,
                color: undefined,
                category: category ? createCategory(category) : undefined,
                usage_count: 0
            };
            
            const id = await this.createTag(tag);
            createdIds.push(id);
        }
        
        return createdIds;
    }

    async mergeTags(primaryTagId: TagID, duplicateTagIds: TagID[]): Promise<void> {
        const primaryTag = this.tags.get(primaryTagId);
        if (!primaryTag) {
            throw new TodoziError(
                TodoziErrorType.NotFoundError,
                `Primary tag with ID "${primaryTagId}" not found`
            );
        }

        for (const duplicateId of duplicateTagIds) {
            const duplicateTag = this.tags.get(duplicateId);
            if (duplicateTag) {
                this.tags.delete(duplicateId);
                
                // Update primary tag usage
                primaryTag.usage_count += duplicateTag.usage_count;
                primaryTag.updated_at = new Date();
                
                // Transfer relationships
                const relationships = this.tagRelationships.get(duplicateId);
                if (relationships) {
                    this.tagRelationships.delete(duplicateId);
                    const primaryRelationships = this.tagRelationships.get(primaryTagId) || [];
                    this.tagRelationships.set(primaryTagId, [
                        ...primaryRelationships,
                        ...relationships.filter(id => id !== primaryTagId)
                    ]);
                }
            }
        }
    }

    private generateUUID(): string {
        return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, (c) => {
            const r = Math.random() * 16 | 0;
            const v = c === 'x' ? r : (r & 0x3 | 0x8);
            return v.toString(16);
        });
    }
}

// TagSearchEngine class
class TagSearchEngine {
    constructor(private tagManager: TagManager) {}

    advancedSearch(query: TagSearchQuery): Tag[] {
        let results = this.tagManager.getAllTags();

        if (query.name_contains) {
            const nameLower = query.name_contains.toLowerCase();
            results = results.filter(tag => 
                tag.name.toLowerCase().includes(nameLower)
            );
        }

        if (query.description_contains) {
            const descLower = query.description_contains.toLowerCase();
            results = results.filter(tag => 
                tag.description ? tag.description.toLowerCase().includes(descLower) : false
            );
        }

        if (query.category) {
            results = results.filter(tag => 
                tag.category ? tag.category === query.category : false
            );
        }

        if (query.min_usage !== undefined) {
            results = results.filter(tag => tag.usage_count >= query.min_usage!);
        }

        if (query.max_usage !== undefined) {
            results = results.filter(tag => tag.usage_count <= query.max_usage!);
        }

        if (query.color) {
            results = results.filter(tag => 
                tag.color ? tag.color === query.color : false
            );
        }

        switch (query.sort_by) {
            case TagSortBy.Name:
                results.sort((a, b) => a.name.localeCompare(b.name));
                break;
            case TagSortBy.Usage:
                results.sort((a, b) => b.usage_count - a.usage_count);
                break;
            case TagSortBy.Created:
                results.sort((a, b) => b.created_at.getTime() - a.created_at.getTime());
                break;
            case TagSortBy.Updated:
                results.sort((a, b) => b.updated_at.getTime() - a.updated_at.getTime());
                break;
        }

        if (query.limit !== undefined) {
            results = results.slice(0, query.limit);
        }

        return results;
    }

    fuzzySearch(query: string, maxDistance: number): [Tag, number][] {
        const queryLower = query.toLowerCase();
        const results: [Tag, number][] = [];

        for (const tag of this.tagManager.getAllTags()) {
            const nameLower = tag.name.toLowerCase();
            const distance = this.levenshteinDistance(queryLower, nameLower);
            
            if (distance <= maxDistance) {
                results.push([tag, distance]);
            }
        }

        results.sort((a, b) => a[1] - b[1]);
        return results;
    }

    getSuggestions(currentTags: string[], limit: number): string[] {
        const suggestions = new Map<string, number>();

        for (const tagName of currentTags) {
            const currentTag = this.tagManager.getTagByName(tagName);
            if (currentTag) {
                const relatedTags = this.tagManager.getRelatedTags(currentTag.id);
                for (const relatedTag of relatedTags) {
                    suggestions.set(
                        relatedTag.name,
                        (suggestions.get(relatedTag.name) || 0) + 1
                    );
                }
            }
        }

        return Array.from(suggestions.entries())
            .sort((a, b) => b[1] - a[1])
            .slice(0, limit)
            .map(([name]) => name);
    }

    private levenshteinDistance(s1: string, s2: string): number {
        const s1Chars = Array.from(s1);
        const s2Chars = Array.from(s2);
        const len1 = s1Chars.length;
        const len2 = s2Chars.length;

        const matrix: number[][] = Array(len1 + 1).fill(null).map(() => Array(len2 + 1).fill(0));

        for (let i = 0; i <= len1; i++) {
            matrix[i][0] = i;
        }

        for (let j = 0; j <= len2; j++) {
            matrix[0][j] = j;
        }

        for (let i = 1; i <= len1; i++) {
            for (let j = 1; j <= len2; j++) {
                const cost = s1Chars[i - 1] === s2Chars[j - 1] ? 0 : 1;
                matrix[i][j] = Math.min(
                    matrix[i - 1][j] + 1,
                    matrix[i][j - 1] + 1,
                    matrix[i - 1][j - 1] + cost
                );
            }
        }

        return matrix[len1][len2];
    }
}

// Tests
class TestRunner {
    static runTests(): void {
        this.testTagManagerCreation();
        this.testTagUpdateBuilder();
        this.testTagStatistics();
        this.testTagSearchQuery();
        this.testLevenshteinDistance();
        
        console.log('All tests passed!');
    }

    private static testTagManagerCreation(): void {
        const manager = new TagManager();
        console.assert(manager.getAllTags().length === 0, 'Manager should be empty');
        console.assert(manager.getAllCategories().length === 0, 'No categories should exist');
    }

    private static testTagUpdateBuilder(): void {
        const update = TagUpdateBuilder.new()
            .withName('New Name')
            .withDescription('New Description')
            .withColor('#FF0000')
            .build();

        console.assert(update.name === 'New Name', 'Name should be set');
        console.assert(update.description === 'New Description', 'Description should be set');
        console.assert(update.color === '#FF0000', 'Color should be set');
    }

    private static testTagStatistics(): void {
        const stats = new TagStatisticsImpl(10, 3, 15, 5.5);
        console.assert(stats.relationships_per_tag() === 1.5, 'Relationships per tag should be 1.5');

        const emptyStats = new TagStatisticsImpl(0, 0, 0, 0);
        console.assert(emptyStats.relationships_per_tag() === 0, 'Empty stats should return 0');
    }

    private static testTagSearchQuery(): void {
        const query = new TagSearchQueryImpl(
            'test',
            undefined,
            'development',
            undefined,
            5,
            undefined,
            TagSortBy.Usage,
            10
        );

        console.assert(query.name_contains === 'test', 'Name contains should be set');
        console.assert(query.category === 'development', 'Category should be set');
        console.assert(query.min_usage === 5, 'Min usage should be set');
        console.assert(query.limit === 10, 'Limit should be set');
    }

    private static testLevenshteinDistance(): void {
        const searchEngine = new TagSearchEngine(new TagManager());
        
        // Using reflection to test private method
        const levenshteinDistance = (searchEngine as any).levenshteinDistance;
        
        console.assert(levenshteinDistance.call(searchEngine, 'kitten', 'kitten') === 0);
        console.assert(levenshteinDistance.call(searchEngine, 'kitten', 'kittens') === 1);
        console.assert(levenshteinDistance.call(searchEngine, 'kitten', 'sitting') === 3);
        console.assert(levenshteinDistance.call(searchEngine, '', 'abc') === 3);
        console.assert(levenshteinDistance.call(searchEngine, 'abc', '') === 3);
    }
}

// Run tests
TestRunner.runTests();

// Export everything
export {
    TagManager,
    TagSearchEngine,
    TagUpdateBuilder,
    TagStatistics,
    TagStatisticsImpl,
    TagSearchQuery,
    TagSearchQueryImpl,
    TagSortBy,
    TodoziError,
    TodoziErrorType,
    TagID,
    Category,
    type Tag,
    type TagUpdate,
    type SearchOptions
};
