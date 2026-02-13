// Module data
const modules = [
    // Core Modules
    { name: 'array', category: 'core', icon: 'fa-list', description: 'Array operations and manipulations', features: ['Sorting', 'Filtering', 'Mapping', 'Reducing'] },
    { name: 'math', category: 'core', icon: 'fa-calculator', description: 'Mathematical functions and calculations', features: ['Basic operations', 'Trigonometry', 'Statistics', 'Random numbers'] },
    { name: 'string', category: 'core', icon: 'fa-font', description: 'String manipulation and processing', features: ['Splitting', 'Joining', 'Regex', 'Encoding'] },
    { name: 'json', category: 'core', icon: 'fa-code', description: 'JSON parsing and generation', features: ['Parse', 'Stringify', 'Validation', 'Pretty print'] },
    
    // System Modules
    { name: 'os', category: 'system', icon: 'fa-desktop', description: 'Operating system interface', features: ['System info', 'Environment variables', 'Paths', 'Commands'] },
    { name: 'process', category: 'system', icon: 'fa-cogs', description: 'Process management', features: ['Spawn', 'Kill', 'Monitor', 'Signals'] },
    { name: 'filesystem', category: 'system', icon: 'fa-folder', description: 'File system operations', features: ['Read/Write', 'Directories', 'Permissions', 'Watch'] },
    { name: 'io', category: 'system', icon: 'fa-exchange-alt', description: 'Input/output operations', features: ['Streams', 'Buffers', 'Pipes', 'Async I/O'] },
    
    // Network Modules
    { name: 'http', category: 'network', icon: 'fa-globe', description: 'HTTP client functions', features: ['GET/POST', 'Headers', 'Cookies', 'Auth'] },
    { name: 'net', category: 'network', icon: 'fa-network-wired', description: 'Network utilities', features: ['TCP/UDP', 'DNS', 'Sockets', 'Protocols'] },
    { name: 'web', category: 'network', icon: 'fa-chrome', description: 'Web development utilities', features: ['Routing', 'Middleware', 'Templates', 'Sessions'] },
    { name: 'web_simple', category: 'network', icon: 'fa-rocket', description: 'Simple web framework', features: ['Quick setup', 'REST API', 'Static files', 'JSON responses'] },
    
    // Utility Modules
    { name: 'crypto', category: 'utilities', icon: 'fa-lock', description: 'Cryptographic functions', features: ['Hashing', 'Encryption', 'Signatures', 'Random'] },
    { name: 'datetime', category: 'utilities', icon: 'fa-clock', description: 'Date/time utilities', features: ['Formatting', 'Parsing', 'Timezones', 'Durations'] },
    { name: 'encoding', category: 'utilities', icon: 'fa-compress', description: 'Text encoding/decoding', features: ['Base64', 'Hex', 'UTF-8', 'Compression'] },
    { name: 'validation', category: 'utilities', icon: 'fa-check-circle', description: 'Data validation utilities', features: ['Email', 'Phone', 'URL', 'Custom rules'] },
    { name: 'database', category: 'utilities', icon: 'fa-database', description: 'Database operations', features: ['SQL', 'Transactions', 'Connection pooling', 'Migrations'] },
    { name: 'config', category: 'utilities', icon: 'fa-cog', description: 'Configuration management', features: ['JSON/YAML', 'Environment', 'Validation', 'Hot reload'] },
    
    // Advanced Modules
    { name: 'ai', category: 'advanced', icon: 'fa-brain', description: 'AI utilities and algorithms', features: ['Neural networks', 'NLP', 'Computer vision', 'Decision trees'] },
    { name: 'graphics', category: 'advanced', icon: 'fa-paint-brush', description: 'Graphics utilities', features: ['2D drawing', 'Canvas', 'SVG', 'Animations'] },
    { name: 'game', category: 'advanced', icon: 'fa-gamepad', description: 'Game development tools', features: ['Physics engine', 'Collision detection', 'Sprites', 'Audio'] },
    { name: 'machine_learning', category: 'advanced', icon: 'fa-robot', description: 'Machine learning', features: ['Regression', 'Classification', 'Clustering', 'Deep learning'] },
    { name: 'image_processing', category: 'advanced', icon: 'fa-image', description: 'Image processing', features: ['Filters', 'Transformations', 'Analysis', 'Format conversion'] },
    
    // Science Modules
    { name: 'physics', category: 'science', icon: 'fa-atom', description: 'Physics calculations', features: ['Mechanics', 'Thermodynamics', 'Waves', 'Quantum'] },
    { name: 'finance', category: 'science', icon: 'fa-chart-line', description: 'Finance calculations', features: ['Interest', 'ROI', 'Risk analysis', 'Portfolio'] },
    { name: 'chemistry', category: 'science', icon: 'fa-flask', description: 'Chemistry calculations', features: ['Molecular weight', 'Reactions', 'Periodic table', 'Solutions'] },
    { name: 'audio', category: 'science', icon: 'fa-volume-up', description: 'Audio processing', features: ['FFT', 'Filters', 'Effects', 'Format conversion'] },
    { name: 'statistics', category: 'science', icon: 'fa-chart-bar', description: 'Statistics', features: ['Descriptive', 'Inferential', 'Regression', 'Hypothesis testing'] },
    
    // System Utilities
    { name: 'logging', category: 'system', icon: 'fa-file-alt', description: 'Logging system', features: ['Levels', 'Formats', 'Outputs', 'Rotation'] },
    { name: 'security', category: 'system', icon: 'fa-shield-alt', description: 'Security functions', features: ['Authentication', 'Authorization', 'Encryption', 'Auditing'] }
];

// Initialize the website
document.addEventListener('DOMContentLoaded', function() {
    renderModules('all');
    setupEventListeners();
    addScrollAnimations();
});

// Render modules based on category
function renderModules(category) {
    const modulesGrid = document.getElementById('modulesGrid');
    const filteredModules = category === 'all' 
        ? modules 
        : modules.filter(module => module.category === category);
    
    modulesGrid.innerHTML = filteredModules.map(module => `
        <div class="module-card" data-category="${module.category}">
            <div class="module-header">
                <div class="module-icon">
                    <i class="fas ${module.icon}"></i>
                </div>
                <div>
                    <div class="module-title">${module.name}.zn</div>
                </div>
            </div>
            <div class="module-category">${module.category}</div>
            <div class="module-description">${module.description}</div>
            <ul class="module-features">
                ${module.features.map(feature => `<li>${feature}</li>`).join('')}
            </ul>
        </div>
    `).join('');
    
    // Add animation to cards
    const cards = modulesGrid.querySelectorAll('.module-card');
    cards.forEach((card, index) => {
        setTimeout(() => {
            card.style.opacity = '0';
            card.style.transform = 'translateY(20px)';
            card.style.transition = 'all 0.5s ease';
            
            setTimeout(() => {
                card.style.opacity = '1';
                card.style.transform = 'translateY(0)';
            }, 50);
        }, index * 50);
    });
}

// Setup event listeners
function setupEventListeners() {
    // Category buttons
    const categoryButtons = document.querySelectorAll('.category-btn');
    categoryButtons.forEach(btn => {
        btn.addEventListener('click', function() {
            // Remove active class from all buttons
            categoryButtons.forEach(b => b.classList.remove('active'));
            // Add active class to clicked button
            this.classList.add('active');
            // Render modules for selected category
            renderModules(this.dataset.category);
        });
    });
    
    // Navigation toggle for mobile
    const navToggle = document.querySelector('.nav-toggle');
    const navMenu = document.querySelector('.nav-menu');
    
    if (navToggle) {
        navToggle.addEventListener('click', function() {
            navMenu.classList.toggle('active');
        });
    }
    
    // Smooth scrolling for navigation links
    const navLinks = document.querySelectorAll('.nav-link');
    navLinks.forEach(link => {
        link.addEventListener('click', function(e) {
            e.preventDefault();
            const targetId = this.getAttribute('href').substring(1);
            const targetSection = document.getElementById(targetId);
            if (targetSection) {
                targetSection.scrollIntoView({ behavior: 'smooth' });
            }
        });
    });
    
    // Module card hover effects
    document.addEventListener('mouseover', function(e) {
        if (e.target.closest('.module-card')) {
            const card = e.target.closest('.module-card');
            card.style.transform = 'translateY(-8px) scale(1.02)';
        }
    });
    
    document.addEventListener('mouseout', function(e) {
        if (e.target.closest('.module-card')) {
            const card = e.target.closest('.module-card');
            card.style.transform = 'translateY(0) scale(1)';
        }
    });
}

// Scroll animations
function addScrollAnimations() {
    const observerOptions = {
        threshold: 0.1,
        rootMargin: '0px 0px -50px 0px'
    };
    
    const observer = new IntersectionObserver(function(entries) {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                entry.target.classList.add('fade-in-up');
            }
        });
    }, observerOptions);
    
    // Observe all sections
    const sections = document.querySelectorAll('section');
    sections.forEach(section => {
        observer.observe(section);
    });
    
    // Observe feature cards
    const featureCards = document.querySelectorAll('.feature-card');
    featureCards.forEach(card => {
        observer.observe(card);
    });
    
    // Observe example cards
    const exampleCards = document.querySelectorAll('.example-card');
    exampleCards.forEach(card => {
        observer.observe(card);
    });
}

// Smooth scroll function
function scrollToSection(sectionId) {
    const section = document.getElementById(sectionId);
    if (section) {
        section.scrollIntoView({ behavior: 'smooth' });
    }
}

// Add typing effect to hero title
function typeWriter() {
    const title = document.querySelector('.hero-title');
    if (title) {
        const text = title.innerHTML;
        title.innerHTML = '';
        let i = 0;
        
        function type() {
            if (i < text.length) {
                title.innerHTML += text.charAt(i);
                i++;
                setTimeout(type, 50);
            }
        }
        
        setTimeout(type, 500);
    }
}

// Add particle effect to hero section
function createParticles() {
    const hero = document.querySelector('.hero');
    if (!hero) return;
    
    for (let i = 0; i < 50; i++) {
        const particle = document.createElement('div');
        particle.className = 'particle';
        particle.style.cssText = `
            position: absolute;
            width: ${Math.random() * 4 + 1}px;
            height: ${Math.random() * 4 + 1}px;
            background: rgba(255, 255, 255, ${Math.random() * 0.5 + 0.2});
            border-radius: 50%;
            left: ${Math.random() * 100}%;
            top: ${Math.random() * 100}%;
            animation: float ${Math.random() * 10 + 10}s linear infinite;
        `;
        hero.appendChild(particle);
    }
}

// Add floating animation
const style = document.createElement('style');
style.textContent = `
    @keyframes float {
        0% {
            transform: translateY(0px) translateX(0px);
            opacity: 0;
        }
        10% {
            opacity: 1;
        }
        90% {
            opacity: 1;
        }
        100% {
            transform: translateY(-100vh) translateX(${Math.random() * 200 - 100}px);
            opacity: 0;
        }
    }
    
    .particle {
        pointer-events: none;
    }
    
    .nav-menu.active {
        display: flex !important;
        position: absolute;
        top: 100%;
        left: 0;
        right: 0;
        background: white;
        flex-direction: column;
        padding: 1rem;
        box-shadow: 0 10px 30px rgba(0, 0, 0, 0.1);
    }
    
    @media (max-width: 768px) {
        .nav-menu {
            display: none;
        }
    }
`;
document.head.appendChild(style);

// Initialize effects
typeWriter();
createParticles();

// Add dynamic year to footer
const footerYear = document.querySelector('.footer-bottom p');
if (footerYear) {
    const currentYear = new Date().getFullYear();
    footerYear.innerHTML = footerYear.innerHTML.replace('2024', currentYear);
}

// Add search functionality for modules
function addModuleSearch() {
    const searchInput = document.createElement('input');
    searchInput.type = 'text';
    searchInput.placeholder = 'Search modules...';
    searchInput.className = 'module-search';
    searchInput.style.cssText = `
        width: 100%;
        max-width: 400px;
        padding: 1rem;
        border: 2px solid #667eea;
        border-radius: 25px;
        margin-bottom: 2rem;
        font-size: 1rem;
        outline: none;
        transition: border-color 0.3s ease;
    `;
    
    searchInput.addEventListener('focus', function() {
        this.style.borderColor = '#5568d3';
    });
    
    searchInput.addEventListener('blur', function() {
        this.style.borderColor = '#667eea';
    });
    
    searchInput.addEventListener('input', function() {
        const searchTerm = this.value.toLowerCase();
        const cards = document.querySelectorAll('.module-card');
        
        cards.forEach(card => {
            const title = card.querySelector('.module-title').textContent.toLowerCase();
            const description = card.querySelector('.module-description').textContent.toLowerCase();
            const features = Array.from(card.querySelectorAll('.module-features li'))
                .map(li => li.textContent.toLowerCase()).join(' ');
            
            if (title.includes(searchTerm) || description.includes(searchTerm) || features.includes(searchTerm)) {
                card.style.display = 'block';
            } else {
                card.style.display = 'none';
            }
        });
    });
    
    const modulesSection = document.querySelector('.modules .container');
    const categoriesDiv = document.querySelector('.module-categories');
    if (modulesSection && categoriesDiv) {
        modulesSection.insertBefore(searchInput, categoriesDiv);
    }
}

// Initialize search
addModuleSearch();
