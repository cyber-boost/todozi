import { TodoziError } from './error.js';

// Date utilities
export class DateUtils {
    static formatDate(date: Date, format: string = 'YYYY-MM-DD HH:mm:ss'): string {
        const year = date.getFullYear();
        const month = String(date.getMonth() + 1).padStart(2, '0');
        const day = String(date.getDate()).padStart(2, '0');
        const hours = String(date.getHours()).padStart(2, '0');
        const minutes = String(date.getMinutes()).padStart(2, '0');
        const seconds = String(date.getSeconds()).padStart(2, '0');

        return format
            .replace('YYYY', year.toString())
            .replace('MM', month)
            .replace('DD', day)
            .replace('HH', hours)
            .replace('mm', minutes)
            .replace('ss', seconds);
    }

    static parseDate(dateString: string): Date {
        const date = new Date(dateString);
        if (isNaN(date.getTime())) {
            throw TodoziError.validation(`Invalid date format: ${dateString}`);
        }
        return date;
    }

    static isToday(date: Date): boolean {
        const today = new Date();
        return date.toDateString() === today.toDateString();
    }

    static isYesterday(date: Date): boolean {
        const yesterday = new Date();
        yesterday.setDate(yesterday.getDate() - 1);
        return date.toDateString() === yesterday.toDateString();
    }

    static isWithinDays(date: Date, days: number): boolean {
        const now = new Date();
        const diffTime = Math.abs(now.getTime() - date.getTime());
        const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24));
        return diffDays <= days;
    }

    static addDays(date: Date, days: number): Date {
        const result = new Date(date);
        result.setDate(result.getDate() + days);
        return result;
    }

    static getRelativeTime(date: Date): string {
        const now = new Date();
        const diffMs = now.getTime() - date.getTime();
        const diffSeconds = Math.floor(diffMs / 1000);
        const diffMinutes = Math.floor(diffSeconds / 60);
        const diffHours = Math.floor(diffMinutes / 60);
        const diffDays = Math.floor(diffHours / 24);

        if (diffSeconds < 60) return 'just now';
        if (diffMinutes < 60) return `${diffMinutes}m ago`;
        if (diffHours < 24) return `${diffHours}h ago`;
        if (diffDays < 7) return `${diffDays}d ago`;
        if (diffDays < 30) return `${Math.floor(diffDays / 7)}w ago`;
        if (diffDays < 365) return `${Math.floor(diffDays / 30)}mo ago`;
        return `${Math.floor(diffDays / 365)}y ago`;
    }
}

// String utilities
export class StringUtils {
    static capitalize(str: string): string {
        return str.charAt(0).toUpperCase() + str.slice(1).toLowerCase();
    }

    static camelCase(str: string): string {
        return str
            .replace(/(?:^\w|[A-Z]|\b\w)/g, (word, index) =>
                index === 0 ? word.toLowerCase() : word.toUpperCase()
            )
            .replace(/\s+/g, '');
    }

    static kebabCase(str: string): string {
        return str
            .replace(/([a-z])([A-Z])/g, '$1-$2')
            .replace(/[\s_]+/g, '-')
            .toLowerCase();
    }

    static snakeCase(str: string): string {
        return str
            .replace(/([a-z])([A-Z])/g, '$1_$2')
            .replace(/[\s-]+/g, '_')
            .toLowerCase();
    }

    static truncate(str: string, maxLength: number, suffix: string = '...'): string {
        if (str.length <= maxLength) return str;
        return str.substring(0, maxLength - suffix.length) + suffix;
    }

    static slugify(str: string): string {
        return str
            .toLowerCase()
            .trim()
            .replace(/[^\w\s-]/g, '')
            .replace(/[\s_-]+/g, '-')
            .replace(/^-+|-+$/g, '');
    }

    static extractHashtags(str: string): string[] {
        const hashtagRegex = /#(\w+)/g;
        const matches = str.match(hashtagRegex);
        return matches ? matches.map(tag => tag.slice(1)) : [];
    }

    static removeMarkdown(str: string): string {
        return str
            .replace(/#{1,6}\s*/g, '') // Headers
            .replace(/\*\*(.*?)\*\*/g, '$1') // Bold
            .replace(/\*(.*?)\*/g, '$1') // Italic
            .replace(/`(.*?)`/g, '$1') // Code
            .replace(/\[([^\]]+)\]\([^\)]+\)/g, '$1') // Links
            .replace(/!\[([^\]]+)\]\([^\)]+\)/g, '$1') // Images
            .replace(/^\s*[-*+]\s+/gm, '') // Lists
            .replace(/^\s*\d+\.\s+/gm, '') // Numbered lists
            .trim();
    }

    static wordCount(str: string): number {
        return str.trim().split(/\s+/).filter(word => word.length > 0).length;
    }

    static similarity(s1: string, s2: string): number {
        const longer = s1.length > s2.length ? s1 : s2;
        const shorter = s1.length > s2.length ? s2 : s1;

        if (longer.length === 0) return 1.0;

        const distance = this.levenshteinDistance(longer, shorter);
        return (longer.length - distance) / longer.length;
    }

    private static levenshteinDistance(s1: string, s2: string): number {
        const matrix = [];

        for (let i = 0; i <= s2.length; i++) {
            matrix[i] = [i];
        }

        for (let j = 0; j <= s1.length; j++) {
            matrix[0][j] = j;
        }

        for (let i = 1; i <= s2.length; i++) {
            for (let j = 1; j <= s1.length; j++) {
                if (s2.charAt(i - 1) === s1.charAt(j - 1)) {
                    matrix[i][j] = matrix[i - 1][j - 1];
                } else {
                    matrix[i][j] = Math.min(
                        matrix[i - 1][j - 1] + 1, // substitution
                        matrix[i][j - 1] + 1,     // insertion
                        matrix[i - 1][j] + 1      // deletion
                    );
                }
            }
        }

        return matrix[s2.length][s1.length];
    }
}

// Array utilities
export class ArrayUtils {
    static unique<T>(arr: T[]): T[] {
        return [...new Set(arr)];
    }

    static uniqueBy<T, K>(arr: T[], keyFn: (item: T) => K): T[] {
        const seen = new Set<K>();
        return arr.filter(item => {
            const key = keyFn(item);
            if (seen.has(key)) return false;
            seen.add(key);
            return true;
        });
    }

    static groupBy<T, K extends string | number | symbol>(
        arr: T[],
        keyFn: (item: T) => K
    ): Record<K, T[]> {
        return arr.reduce((groups, item) => {
            const key = keyFn(item);
            if (!groups[key]) groups[key] = [];
            groups[key].push(item);
            return groups;
        }, {} as Record<K, T[]>);
    }

    static sortBy<T>(arr: T[], keyFn: (item: T) => any, order: 'asc' | 'desc' = 'asc'): T[] {
        return [...arr].sort((a, b) => {
            const aVal = keyFn(a);
            const bVal = keyFn(b);

            if (aVal < bVal) return order === 'asc' ? -1 : 1;
            if (aVal > bVal) return order === 'asc' ? 1 : -1;
            return 0;
        });
    }

    static chunk<T>(arr: T[], size: number): T[][] {
        const chunks: T[][] = [];
        for (let i = 0; i < arr.length; i += size) {
            chunks.push(arr.slice(i, i + size));
        }
        return chunks;
    }

    static flatten<T>(arr: T[][]): T[] {
        return arr.reduce((flat, subArr) => flat.concat(subArr), []);
    }

    static intersection<T>(arr1: T[], arr2: T[]): T[] {
        return arr1.filter(item => arr2.includes(item));
    }

    static difference<T>(arr1: T[], arr2: T[]): T[] {
        return arr1.filter(item => !arr2.includes(item));
    }

    static shuffle<T>(arr: T[]): T[] {
        const shuffled = [...arr];
        for (let i = shuffled.length - 1; i > 0; i--) {
            const j = Math.floor(Math.random() * (i + 1));
            [shuffled[i], shuffled[j]] = [shuffled[j], shuffled[i]];
        }
        return shuffled;
    }

    static sample<T>(arr: T[], count: number = 1): T[] {
        if (count >= arr.length) return [...arr];
        const shuffled = this.shuffle(arr);
        return shuffled.slice(0, count);
    }
}

// Object utilities
export class ObjectUtils {
    static isEmpty(obj: any): boolean {
        if (obj == null) return true;
        if (typeof obj !== 'object') return false;
        if (Array.isArray(obj)) return obj.length === 0;
        return Object.keys(obj).length === 0;
    }

    static deepClone<T>(obj: T): T {
        if (obj === null || typeof obj !== 'object') return obj;
        if (obj instanceof Date) return new Date(obj.getTime()) as unknown as T;
        if (Array.isArray(obj)) return obj.map(item => this.deepClone(item)) as unknown as T;

        const cloned = {} as T;
        for (const key in obj) {
            if (obj.hasOwnProperty(key)) {
                cloned[key] = this.deepClone(obj[key]);
            }
        }
        return cloned;
    }

    static pick<T, K extends keyof T>(obj: T, keys: K[]): Pick<T, K> {
        const result = {} as Pick<T, K>;
        keys.forEach(key => {
            if (key in obj) {
                result[key] = obj[key];
            }
        });
        return result;
    }

    static omit<T, K extends keyof T>(obj: T, keys: K[]): Omit<T, K> {
        const result = { ...obj };
        keys.forEach(key => delete result[key]);
        return result;
    }

    static merge<T extends Record<string, any>>(target: T, source: Partial<T>): T {
        return { ...target, ...source };
    }

    static deepMerge<T extends Record<string, any>>(target: T, source: Partial<T>): T {
        const result = { ...target };

        for (const key in source) {
            if (source.hasOwnProperty(key)) {
                const sourceValue = source[key];
                const targetValue = result[key];

                if (this.isObject(sourceValue) && this.isObject(targetValue)) {
                    result[key] = this.deepMerge(targetValue, sourceValue);
                } else {
                    result[key] = sourceValue as T[Extract<keyof T, string>];
                }
            }
        }

        return result;
    }

    private static isObject(item: any): item is Record<string, any> {
        return item && typeof item === 'object' && !Array.isArray(item);
    }

    static get<T>(obj: any, path: string, defaultValue?: T): T | undefined {
        const keys = path.split('.');
        let current = obj;

        for (const key of keys) {
            if (current == null || typeof current !== 'object') {
                return defaultValue;
            }
            current = current[key];
        }

        return current !== undefined ? current : defaultValue;
    }

    static set(obj: any, path: string, value: any): void {
        const keys = path.split('.');
        const lastKey = keys.pop()!;
        let current = obj;

        for (const key of keys) {
            if (!(key in current) || typeof current[key] !== 'object' || current[key] === null) {
                current[key] = {};
            }
            current = current[key];
        }

        current[lastKey] = value;
    }
}

// Validation utilities
export class ValidationUtils {
    static isEmail(email: string): boolean {
        const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
        return emailRegex.test(email);
    }

    static isUrl(url: string): boolean {
        try {
            new URL(url);
            return true;
        } catch {
            return false;
        }
    }

    static isUUID(uuid: string): boolean {
        const uuidRegex = /^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i;
        return uuidRegex.test(uuid);
    }

    static isValidPriority(priority: string): boolean {
        return ['low', 'medium', 'high', 'critical', 'urgent'].includes(priority.toLowerCase());
    }

    static isValidStatus(status: string): boolean {
        return ['todo', 'in_progress', 'blocked', 'review', 'done', 'cancelled', 'deferred'].includes(status.toLowerCase());
    }

    static validateRequired(value: any, fieldName: string): void {
        if (value == null || value === '' || (Array.isArray(value) && value.length === 0)) {
            throw TodoziError.validation(`${fieldName} is required`);
        }
    }

    static validateStringLength(value: string, fieldName: string, min?: number, max?: number): void {
        if (min !== undefined && value.length < min) {
            throw TodoziError.validation(`${fieldName} must be at least ${min} characters`);
        }
        if (max !== undefined && value.length > max) {
            throw TodoziError.validation(`${fieldName} must be at most ${max} characters`);
        }
    }

    static validateNumberRange(value: number, fieldName: string, min?: number, max?: number): void {
        if (min !== undefined && value < min) {
            throw TodoziError.validation(`${fieldName} must be at least ${min}`);
        }
        if (max !== undefined && value > max) {
            throw TodoziError.validation(`${fieldName} must be at most ${max}`);
        }
    }

    static validateEnum(value: string, fieldName: string, validValues: string[]): void {
        if (!validValues.includes(value.toLowerCase())) {
            throw TodoziError.validation(`${fieldName} must be one of: ${validValues.join(', ')}`);
        }
    }
}

// Async utilities
export class AsyncUtils {
    static async delay(ms: number): Promise<void> {
        return new Promise(resolve => setTimeout(resolve, ms));
    }

    static async timeout<T>(promise: Promise<T>, ms: number): Promise<T> {
        return Promise.race([
            promise,
            new Promise<never>((_, reject) =>
                setTimeout(() => reject(new Error('Operation timed out')), ms)
            )
        ]);
    }

    static async retry<T>(
        fn: () => Promise<T>,
        maxAttempts: number = 3,
        delay: number = 1000
    ): Promise<T> {
        let lastError: Error;

        for (let attempt = 1; attempt <= maxAttempts; attempt++) {
            try {
                return await fn();
            } catch (error) {
                lastError = error as Error;

                if (attempt < maxAttempts) {
                    await this.delay(delay * attempt);
                }
            }
        }

        throw lastError!;
    }

    static async parallelLimit<T, R>(
        items: T[],
        fn: (item: T) => Promise<R>,
        limit: number
    ): Promise<R[]> {
        const results: R[] = [];
        const batches = ArrayUtils.chunk(items, limit);

        for (const batch of batches) {
            const batchResults = await Promise.all(batch.map(fn));
            results.push(...batchResults);
        }

        return results;
    }
}

// ID generation utilities
export class IdUtils {
    static generateUUID(): string {
        return crypto.randomUUID();
    }

    static generateShortId(length: number = 8): string {
        const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
        let result = '';
        for (let i = 0; i < length; i++) {
            result += chars.charAt(Math.floor(Math.random() * chars.length));
        }
        return result;
    }

    static generateTimestampId(): string {
        return `${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
    }
}

// Export utility namespace
export const Utils = {
    Date: DateUtils,
    String: StringUtils,
    Array: ArrayUtils,
    Object: ObjectUtils,
    Validation: ValidationUtils,
    Async: AsyncUtils,
    Id: IdUtils
};

export default Utils;
