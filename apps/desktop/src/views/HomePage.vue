<template>
  <div class="home-page">
    <!-- Hero Section -->
    <section class="hero-section">
      <div class="hero-background">
        <div class="hero-gradient"></div>
      </div>
      
      <div class="hero-content">
        <div class="hero-text">
          <h1 class="hero-title">
            Track Your Favorite
            <span class="gradient-text">Movies & TV Shows</span>
          </h1>
          <p class="hero-subtitle">
            Discover trending content, build your watchlist, and never miss what's worth watching.
          </p>
        </div>
        
        <div class="hero-actions">
          <router-link to="/popular" class="cta-button primary">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M12 2L15.09 8.26L22 9.27L17 14.14L18.18 21.02L12 17.77L5.82 21.02L7 14.14L2 9.27L8.91 8.26L12 2Z" fill="currentColor"/>
            </svg>
            Explore Popular
          </router-link>
          
          <router-link to="/watchlist" class="cta-button secondary">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M19 14C19 18.4183 15.4183 22 11 22C6.58172 22 3 18.4183 3 14C3 9.58172 6.58172 6 11 6C15.4183 6 19 9.58172 19 14Z" stroke="currentColor" stroke-width="2"/>
              <path d="M9 11L11 13L15 9" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            My Watchlist
          </router-link>
          
          <router-link to="/watched" class="cta-button tertiary">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M12 2L15.09 8.26L22 9.27L17 14.14L18.18 21.02L12 17.77L5.82 21.02L7 14.14L2 9.27L8.91 8.26L12 2Z" fill="currentColor"/>
            </svg>
            Watched
          </router-link>
        </div>
      </div>
      
      <div class="hero-stats">
        <div class="stat-card">
          <div class="stat-icon">
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M7 4V2C7 1.44772 7.44772 1 8 1H16C16.5523 1 17 1.44772 17 2V4M7 4H5C3.89543 4 3 4.89543 3 6V20C3 21.1046 3.89543 22 5 22H19C20.1046 22 21 21.1046 21 20V6C21 4.89543 20.1046 4 19 4H17M7 4H17M9 9H15M9 13H15M9 17H15" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </div>
          <div class="stat-content">
            <div class="stat-number">10K+</div>
            <div class="stat-label">Movies</div>
          </div>
        </div>
        
        <div class="stat-card">
          <div class="stat-icon">
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M7 21H17C18.1046 21 19 20.1046 19 19V5C19 3.89543 18.1046 3 17 3H7C5.89543 3 5 3.89543 5 5V19C5 20.1046 5.89543 21 7 21ZM9 9H15V15H9V9Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </div>
          <div class="stat-content">
            <div class="stat-number">5K+</div>
            <div class="stat-label">TV Shows</div>
          </div>
        </div>
        
        <div class="stat-card">
          <div class="stat-icon">
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M12 2L15.09 8.26L22 9.27L17 14.14L18.18 21.02L12 17.77L5.82 21.02L7 14.14L2 9.27L8.91 8.26L12 2Z" fill="currentColor"/>
            </svg>
          </div>
          <div class="stat-content">
            <div class="stat-number">Daily</div>
            <div class="stat-label">Updates</div>
          </div>
        </div>
      </div>
    </section>

    <!-- Features Section -->
    <section class="features-section">
      <div class="container">
        <h2 class="section-title">Everything You Need</h2>
        <div class="features-grid">
          <div class="feature-card">
            <div class="feature-icon">
              <svg width="32" height="32" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M21 21L16.514 16.506L21 21ZM19 10.5C19 15.194 15.194 19 10.5 19C5.806 19 2 15.194 2 10.5C2 5.806 5.806 2 10.5 2C15.194 2 19 5.806 19 10.5Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            </div>
            <h3 class="feature-title">Smart Search</h3>
            <p class="feature-description">
              Find any movie or TV show instantly with our powerful search engine.
            </p>
          </div>
          
          <div class="feature-card">
            <div class="feature-icon">
              <svg width="32" height="32" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M19 14C19 18.4183 15.4183 22 11 22C6.58172 22 3 18.4183 3 14C3 9.58172 6.58172 6 11 6C15.4183 6 19 9.58172 19 14Z" stroke="currentColor" stroke-width="2"/>
                <path d="M9 11L11 13L15 9" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            </div>
            <h3 class="feature-title">Personal Watchlist</h3>
            <p class="feature-description">
              Keep track of what you want to watch and organize your entertainment.
            </p>
          </div>
          
          <div class="feature-card">
            <div class="feature-icon">
              <svg width="32" height="32" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M13 2L3 14H12L11 22L21 10H12L13 2Z" fill="currentColor"/>
              </svg>
            </div>
            <h3 class="feature-title">Lightning Fast</h3>
            <p class="feature-description">
              Built with Tauri for native performance and responsiveness.
            </p>
          </div>
        </div>
      </div>
    </section>

    <!-- Quick Actions Section -->
    <section class="quick-actions-section">
      <div class="container">
        <h2 class="section-title">Quick Actions</h2>
        <div class="quick-actions-grid">
          <router-link to="/popular" class="quick-action-card">
            <div class="quick-action-icon">
              <svg width="28" height="28" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M12 2L15.09 8.26L22 9.27L17 14.14L18.18 21.02L12 17.77L5.82 21.02L7 14.14L2 9.27L8.91 8.26L12 2Z" fill="currentColor"/>
              </svg>
            </div>
            <div class="quick-action-content">
              <h3>Trending Movies</h3>
              <p>Discover what's popular right now</p>
            </div>
            <div class="quick-action-arrow">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M5 12H19M19 12L12 5M19 12L12 19" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            </div>
          </router-link>
          
          <router-link to="/popular" class="quick-action-card">
            <div class="quick-action-icon">
              <svg width="28" height="28" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M7 21H17C18.1046 21 19 20.1046 19 19V5C19 3.89543 18.1046 3 17 3H7C5.89543 3 5 3.89543 5 5V19C5 20.1046 5.89543 21 7 21ZM9 9H15V15H9V9Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            </div>
            <div class="quick-action-content">
              <h3>Popular TV Shows</h3>
              <p>Binge-worthy series trending now</p>
            </div>
            <div class="quick-action-arrow">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M5 12H19M19 12L12 5M19 12L12 19" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            </div>
          </router-link>
          
          <router-link to="/watchlist" class="quick-action-card">
            <div class="quick-action-icon">
              <svg width="28" height="28" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M19 14C19 18.4183 15.4183 22 11 22C6.58172 22 3 18.4183 3 14C3 9.58172 6.58172 6 11 6C15.4183 6 19 9.58172 19 14Z" stroke="currentColor" stroke-width="2"/>
                <path d="M9 11L11 13L15 9" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            </div>
            <div class="quick-action-content">
              <h3>My Watchlist</h3>
              <p>View and manage your saved content</p>
            </div>
            <div class="quick-action-arrow">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M5 12H19M19 12L12 5M19 12L12 19" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            </div>
          </router-link>
          
          <router-link to="/watched" class="quick-action-card">
            <div class="quick-action-icon">
              <svg width="28" height="28" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M12 2L15.09 8.26L22 9.27L17 14.14L18.18 21.02L12 17.77L5.82 21.02L7 14.14L2 9.27L8.91 8.26L12 2Z" fill="currentColor"/>
              </svg>
            </div>
            <div class="quick-action-content">
              <h3>Watched Content</h3>
              <p>Your rated movies and TV shows</p>
            </div>
            <div class="quick-action-arrow">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M5 12H19M19 12L12 5M19 12L12 19" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            </div>
          </router-link>
          
          <router-link to="/test-markdown" class="quick-action-card">
            <div class="quick-action-icon">
              <svg width="28" height="28" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M14 2H6C4.89543 2 4 2.89543 4 4V20C4 21.1046 4.89543 22 6 22H18C19.1046 22 20 21.1046 20 20V8L14 2Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <polyline points="14,2 14,8 20,8" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <line x1="16" y1="13" x2="8" y2="13" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <line x1="16" y1="17" x2="8" y2="17" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            </div>
            <div class="quick-action-content">
              <h3>Test Markdown</h3>
              <p>Try out the markdown notes feature</p>
            </div>
            <div class="quick-action-arrow">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M5 12H19M19 12L12 5M19 12L12 19" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            </div>
          </router-link>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
// Add any reactive data or methods here if needed
</script>

<style scoped>
.home-page {
  min-height: 100vh;
  background: var(--color-background);
}

/* Hero Section */
.hero-section {
  position: relative;
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  text-align: center;
  padding: var(--spacing-2xl) var(--spacing-md);
  overflow: hidden;
}

.hero-background {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: 
    radial-gradient(circle at 20% 50%, rgba(35, 134, 54, 0.15) 0%, transparent 50%),
    radial-gradient(circle at 80% 20%, rgba(31, 111, 235, 0.15) 0%, transparent 50%),
    radial-gradient(circle at 40% 80%, rgba(168, 85, 247, 0.1) 0%, transparent 50%);
}

.hero-gradient {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(
    45deg,
    var(--color-background) 0%,
    var(--color-background-secondary) 50%,
    var(--color-background) 100%
  );
  opacity: 0.8;
}

.hero-content {
  position: relative;
  z-index: 2;
  max-width: 800px;
  margin: 0 auto;
}

.hero-title {
  font-size: 3.5rem;
  font-weight: 700;
  line-height: 1.2;
  margin-bottom: var(--spacing-lg);
  color: var(--color-text-primary);
}

.gradient-text {
  background: linear-gradient(135deg, var(--color-accent-primary), var(--color-accent-secondary));
  background-clip: text;
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  display: inline-block;
}

.hero-subtitle {
  font-size: 1.25rem;
  color: var(--color-text-secondary);
  margin-bottom: var(--spacing-2xl);
  line-height: 1.6;
}

.hero-actions {
  display: flex;
  gap: var(--spacing-lg);
  justify-content: center;
  flex-wrap: wrap;
  margin-bottom: var(--spacing-2xl);
}

.cta-button {
  display: inline-flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-md) var(--spacing-xl);
  border-radius: var(--radius-large);
  font-weight: 600;
  font-size: 1.125rem;
  text-decoration: none;
  transition: all var(--transition-medium);
  border: 2px solid transparent;
  min-width: 180px;
  justify-content: center;
}

.cta-button.primary {
  background: var(--color-accent-primary);
  color: white;
  box-shadow: var(--shadow-medium);
}

.cta-button.primary:hover {
  background: var(--color-accent-primary-hover);
  transform: translateY(-2px);
  box-shadow: var(--shadow-large);
}

.cta-button.secondary {
  background: transparent;
  color: var(--color-text-primary);
  border-color: var(--color-border);
}

.cta-button.secondary:hover {
  background: var(--color-surface);
  border-color: var(--color-accent-secondary);
  color: var(--color-accent-secondary);
  transform: translateY(-2px);
}

.cta-button.tertiary {
  background: var(--color-surface);
  color: var(--color-text-secondary);
  border-color: var(--color-border);
}

.cta-button.tertiary:hover {
  background: var(--color-background);
  border-color: var(--color-primary);
  color: var(--color-primary);
  transform: translateY(-2px);
}

.hero-stats {
  display: flex;
  gap: var(--spacing-xl);
  justify-content: center;
  flex-wrap: wrap;
  position: relative;
  z-index: 2;
}

.stat-card {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
  background: var(--color-surface);
  padding: var(--spacing-lg);
  border-radius: var(--radius-large);
  border: 1px solid var(--color-border);
  backdrop-filter: blur(8px);
}

.stat-icon {
  color: var(--color-accent-primary);
}

.stat-number {
  font-size: 1.5rem;
  font-weight: 700;
  color: var(--color-text-primary);
  line-height: 1;
}

.stat-label {
  font-size: 0.875rem;
  color: var(--color-text-secondary);
}

/* Features Section */
.features-section {
  padding: var(--spacing-2xl) var(--spacing-md);
  background: var(--color-background-secondary);
}

.section-title {
  text-align: center;
  font-size: 2.5rem;
  font-weight: 700;
  margin-bottom: var(--spacing-2xl);
  color: var(--color-text-primary);
}

.features-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: var(--spacing-xl);
  max-width: 1200px;
  margin: 0 auto;
}

.feature-card {
  background: var(--color-background);
  padding: var(--spacing-xl);
  border-radius: var(--radius-large);
  border: 1px solid var(--color-border);
  text-align: center;
  transition: all var(--transition-medium);
}

.feature-card:hover {
  transform: translateY(-4px);
  box-shadow: var(--shadow-large);
  border-color: var(--color-accent-primary);
}

.feature-icon {
  color: var(--color-accent-primary);
  margin-bottom: var(--spacing-lg);
}

.feature-title {
  font-size: 1.5rem;
  font-weight: 600;
  margin-bottom: var(--spacing-md);
  color: var(--color-text-primary);
}

.feature-description {
  color: var(--color-text-secondary);
  line-height: 1.6;
  margin: 0;
}

/* Quick Actions Section */
.quick-actions-section {
  padding: var(--spacing-2xl) var(--spacing-md);
}

.quick-actions-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
  gap: var(--spacing-lg);
  max-width: 1200px;
  margin: 0 auto;
}

.quick-action-card {
  display: flex;
  align-items: center;
  gap: var(--spacing-lg);
  background: var(--color-card-background);
  border: 1px solid var(--color-card-border);
  border-radius: var(--radius-large);
  padding: var(--spacing-xl);
  text-decoration: none;
  transition: all var(--transition-medium);
  position: relative;
  overflow: hidden;
}

.quick-action-card::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.05), transparent);
  transition: left var(--transition-medium);
}

.quick-action-card:hover::before {
  left: 100%;
}

.quick-action-card:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-medium);
  border-color: var(--color-accent-primary);
}

.quick-action-icon {
  color: var(--color-accent-primary);
  flex-shrink: 0;
}

.quick-action-content {
  flex: 1;
}

.quick-action-content h3 {
  font-size: 1.25rem;
  font-weight: 600;
  margin-bottom: var(--spacing-xs);
  color: var(--color-text-primary);
}

.quick-action-content p {
  color: var(--color-text-secondary);
  margin: 0;
  font-size: 0.875rem;
}

.quick-action-arrow {
  color: var(--color-text-muted);
  transition: all var(--transition-fast);
  flex-shrink: 0;
}

.quick-action-card:hover .quick-action-arrow {
  color: var(--color-accent-primary);
  transform: translateX(4px);
}

/* Responsive Design */
@media (max-width: 768px) {
  .hero-title {
    font-size: 2.5rem;
  }
  
  .hero-subtitle {
    font-size: 1.125rem;
  }
  
  .hero-actions {
    flex-direction: column;
    align-items: center;
  }
  
  .cta-button {
    width: 100%;
    max-width: 280px;
  }
  
  .hero-stats {
    flex-direction: column;
    align-items: center;
  }
  
  .stat-card {
    width: 100%;
    max-width: 240px;
  }
  
  .section-title {
    font-size: 2rem;
  }
  
  .features-grid {
    grid-template-columns: 1fr;
  }
  
  .quick-actions-grid {
    grid-template-columns: 1fr;
  }
  
  .quick-action-card {
    flex-direction: column;
    text-align: center;
    gap: var(--spacing-md);
  }
}

@media (max-width: 480px) {
  .hero-section {
    padding: var(--spacing-xl) var(--spacing-sm);
  }
  
  .hero-title {
    font-size: 2rem;
  }
  
  .hero-subtitle {
    font-size: 1rem;
  }
  
  .features-section,
  .quick-actions-section {
    padding: var(--spacing-xl) var(--spacing-sm);
  }
  
  .feature-card,
  .quick-action-card {
    padding: var(--spacing-lg);
  }
}
</style>