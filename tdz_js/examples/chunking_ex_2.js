// example2.js - Web Scraper Project Implementation
import { 
    CodeGenerationGraph, 
    ChunkingLevel, 
    ChunkStatus 
} from '../todozi/chunking.js';

// Initialize the code generation graph with 10000 line limit
const graph = new CodeGenerationGraph(10000);

// Define the project structure using chunks
graph.addChunk('project_setup', ChunkingLevel.Project, []);
graph.addChunk('database_module', ChunkingLevel.Module, ['project_setup']);
graph.addChunk('scraper_module', ChunkingLevel.Module, ['project_setup']);
graph.addChunk('main_module', ChunkingLevel.Module, ['database_module', 'scraper_module']);
graph.addChunk('db_connection_class', ChunkingLevel.Class, ['database_module']);
graph.addChunk('db_operations_class', ChunkingLevel.Class, ['database_module', 'db_connection_class']);
graph.addChunk('scraper_class', ChunkingLevel.Class, ['scraper_module']);
graph.addChunk('main_function', ChunkingLevel.Method, ['main_module', 'db_operations_class', 'scraper_class']);

// Implement each chunk
graph.updateChunkCode('project_setup', `
# Web Scraper with Database Storage
# Project to scrape product data and store in SQLite database
`);

graph.updateChunkCode('database_module', `
"""
Database handling module for web scraper
Contains classes for database connection and operations
"""
`);

graph.updateChunkCode('scraper_module', `
"""
Web scraping module
Contains classes for scraping product data from websites
"""
`);

graph.updateChunkCode('main_module', `
"""
Main application module
Coordinates scraping and database operations
"""
`);

graph.updateChunkCode('db_connection_class', `
class DatabaseConnection:
    """Handles SQLite database connections"""
    
    def __init__(self, db_path):
        self.db_path = db_path
        self.connection = None
    
    def connect(self):
        """Establish database connection"""
        import sqlite3
        self.connection = sqlite3.connect(self.db_path)
        return self.connection
    
    def disconnect(self):
        """Close database connection"""
        if self.connection:
            self.connection.close()
`);

graph.updateChunkCode('db_operations_class', `
class DatabaseOperations:
    """Handles database operations for scraped data"""
    
    def __init__(self, db_connection):
        self.db = db_connection
    
    def create_table(self):
        """Create products table if not exists"""
        cursor = self.db.connection.cursor()
        cursor.execute('''
            CREATE TABLE IF NOT EXISTS products (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                price REAL NOT NULL,
                url TEXT UNIQUE NOT NULL
            )
        ''')
        self.db.connection.commit()
    
    def insert_product(self, name, price, url):
        """Insert product data into database"""
        cursor = self.db.connection.cursor()
        try:
            cursor.execute(
                'INSERT INTO products (name, price, url) VALUES (?, ?, ?)',
                (name, price, url)
            )
            self.db.connection.commit()
            return True
        except sqlite3.IntegrityError:
            return False  # Duplicate URL
`);

graph.updateChunkCode('scraper_class', `
import requests
from bs4 import BeautifulSoup

class WebScraper:
    """Handles web scraping operations"""
    
    def __init__(self, base_url):
        self.base_url = base_url
        self.session = requests.Session()
        self.session.headers.update({
            'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36'
        })
    
    def scrape_product(self, product_url):
        """Scrape product information from URL"""
        try:
            response = self.session.get(product_url)
            response.raise_for_status()
            
            soup = BeautifulSoup(response.content, 'html.parser')
            
            # Example selectors (would need to be customized per site)
            name_elem = soup.find('h1', class_='product-title')
            price_elem = soup.find('span', class_='price')
            
            name = name_elem.text.strip() if name_elem else 'Unknown'
            price_text = price_elem.text.strip() if price_elem else '0.0'
            price = float(''.join(filter(str.isdigit, price_text))) / 100
            
            return {
                'name': name,
                'price': price,
                'url': product_url
            }
        except Exception as e:
            print(f"Error scraping {product_url}: {e}")
            return None
`);

graph.updateChunkCode('main_function', `
def main():
    """Main application function"""
    # Initialize components
    db_conn = DatabaseConnection('products.db')
    db_conn.connect()
    
    db_ops = DatabaseOperations(db_conn)
    db_ops.create_table()
    
    scraper = WebScraper('https://example-store.com')
    
    # Example product URLs to scrape
    product_urls = [
        'https://example-store.com/product/1',
        'https://example-store.com/product/2',
        # ... more URLs
    ]
    
    # Scrape and store products
    for url in product_urls:
        product = scraper.scrape_product(url)
        if product:
            success = db_ops.insert_product(
                product['name'], 
                product['price'], 
                product['url']
            )
            status = "Added" if success else "Duplicate"
            print(f"{status}: {product['name']} - ${product['price']}")

if __name__ == "__main__":
    main()
`);

// Mark chunks as completed
graph.markChunkCompleted('project_setup');
graph.markChunkCompleted('database_module');
graph.markChunkCompleted('scraper_module');
graph.markChunkCompleted('main_module');
graph.markChunkCompleted('db_connection_class');
graph.markChunkCompleted('db_operations_class');
graph.markChunkCompleted('scraper_class');
graph.markChunkCompleted('main_function');

// Validate implementation
graph.markChunkValidated('project_setup');
graph.markChunkValidated('database_module');
graph.markChunkValidated('scraper_module');
graph.markChunkValidated('main_module');
graph.markChunkValidated('db_connection_class');
graph.markChunkValidated('db_operations_class');
graph.markChunkValidated('scraper_class');
graph.markChunkValidated('main_function');

// Display project summary
console.log(graph.getProjectSummary());

// Show dependency chain for main function
console.log("Dependency chain for main function:");
console.log(graph.getDependencyChain('main_function'));

/*
Here's a practical example demonstrating how to use the chunking system to build a web scraper with database storage:

This example demonstrates:

1. **Project Structure Definition**:
   - Creating a dependency graph for a web scraper project
   - Organizing code into modules (database, scraper, main)
   - Defining classes and functions with proper dependencies

2. **Chunk Implementation**:
   - Each chunk contains code at the appropriate abstraction level
   - Project-level chunk defines the overall purpose
   - Module-level chunks define component responsibilities
   - Class-level chunks implement specific functionality
   - Method-level chunk coordinates the application flow

3. **Dependency Management**:
   - Properly defined dependencies between components
   - Automatic dependency resolution using the graph
   - Validation of implementation completeness

4. **Project Tracking**:
   - Line count tracking through the project state
   - Completion status management
   - Dependency chain visualization

Key benefits shown in this example:
- Clear separation of concerns
- Automatic dependency resolution
- Progress tracking at multiple levels
- Modular code organization
- Easy to extend and maintain structure

To run this example:
1. Save as `example2.js`
2. Ensure `chunking.js` is in the same directory
3. Run with: `node example2.js`

The output will show the project summary including:
- Total chunks and completion status
- Project state with line counts
- Context window information
- Dependency chain for the main function
*/