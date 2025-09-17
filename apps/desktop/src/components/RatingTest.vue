<template>
  <div class="rating-test">
    <h2>Star Rating Test Component</h2>
    
    <div class="test-section">
      <h3>Interactive Rating</h3>
      <p>Current rating: {{ testRating }}</p>
      <StarRating
        v-model="testRating"
        @change="onRatingChange"
        @clear="onRatingClear"
        :show-rating-text="true"
        :show-clear-button="true"
        size="large"
      />
    </div>

    <div class="test-section">
      <h3>Preset Ratings</h3>
      <div class="preset-buttons">
        <button @click="setRating(0.5)">0.5 Star</button>
        <button @click="setRating(1.0)">1 Star</button>
        <button @click="setRating(2.5)">2.5 Stars</button>
        <button @click="setRating(4.0)">4 Stars</button>
        <button @click="setRating(5.0)">5 Stars</button>
      </div>
    </div>

    <div class="test-section">
      <h3>Readonly Examples</h3>
      <div class="readonly-examples">
        <div class="example">
          <span>2.5 Stars:</span>
          <StarRating
            :model-value="2.5"
            :readonly="true"
            :show-rating-text="true"
            :show-clear-button="false"
            size="medium"
          />
        </div>
        <div class="example">
          <span>4.5 Stars:</span>
          <StarRating
            :model-value="4.5"
            :readonly="true"
            :show-rating-text="true"
            :show-clear-button="false"
            size="medium"
          />
        </div>
        <div class="example">
          <span>5.0 Stars:</span>
          <StarRating
            :model-value="5.0"
            :readonly="true"
            :show-rating-text="true"
            :show-clear-button="false"
            size="medium"
          />
        </div>
      </div>
    </div>

    <div class="test-section">
      <h3>Different Sizes</h3>
      <div class="size-examples">
        <div class="size-example">
          <span>Small:</span>
          <StarRating
            :model-value="3.5"
            :readonly="true"
            :show-rating-text="true"
            :show-clear-button="false"
            size="small"
          />
        </div>
        <div class="size-example">
          <span>Medium:</span>
          <StarRating
            :model-value="3.5"
            :readonly="true"
            :show-rating-text="true"
            :show-clear-button="false"
            size="medium"
          />
        </div>
        <div class="size-example">
          <span>Large:</span>
          <StarRating
            :model-value="3.5"
            :readonly="true"
            :show-rating-text="true"
            :show-clear-button="false"
            size="large"
          />
        </div>
      </div>
    </div>

    <div class="debug-info">
      <h3>Debug Info</h3>
      <p>Last action: {{ lastAction }}</p>
      <p>Rating history: {{ ratingHistory.join(', ') }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import StarRating from './StarRating.vue'

const testRating = ref(0)
const lastAction = ref('None')
const ratingHistory = ref<number[]>([])

const onRatingChange = (rating: number) => {
  console.log('Rating changed to:', rating)
  lastAction.value = `Rated ${rating} stars`
  ratingHistory.value.push(rating)
  if (ratingHistory.value.length > 5) {
    ratingHistory.value.shift()
  }
}

const onRatingClear = () => {
  console.log('Rating cleared')
  lastAction.value = 'Rating cleared'
  ratingHistory.value.push(0)
  if (ratingHistory.value.length > 5) {
    ratingHistory.value.shift()
  }
}

const setRating = (rating: number) => {
  testRating.value = rating
  lastAction.value = `Set to ${rating} stars via button`
}
</script>

<style scoped>
.rating-test {
  padding: 24px;
  max-width: 800px;
  margin: 0 auto;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
}

.rating-test h2 {
  color: #1f2937;
  margin-bottom: 32px;
  text-align: center;
}

.test-section {
  margin-bottom: 32px;
  padding: 20px;
  background: #f9fafb;
  border-radius: 8px;
  border: 1px solid #e5e7eb;
}

.test-section h3 {
  color: #374151;
  margin: 0 0 16px 0;
  font-size: 1.25rem;
}

.preset-buttons {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
}

.preset-buttons button {
  padding: 8px 16px;
  background: #3b82f6;
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
  transition: background-color 0.2s;
}

.preset-buttons button:hover {
  background: #2563eb;
}

.readonly-examples,
.size-examples {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.example,
.size-example {
  display: flex;
  align-items: center;
  gap: 16px;
}

.example span,
.size-example span {
  min-width: 80px;
  font-weight: 500;
  color: #4b5563;
}

.debug-info {
  background: #eff6ff;
  padding: 16px;
  border-radius: 8px;
  border: 1px solid #bfdbfe;
}

.debug-info h3 {
  color: #1e40af;
  margin: 0 0 12px 0;
}

.debug-info p {
  margin: 4px 0;
  color: #1e40af;
  font-family: 'Monaco', 'Menlo', monospace;
  font-size: 13px;
}

/* Dark theme support */
@media (prefers-color-scheme: dark) {
  .rating-test h2 {
    color: #f9fafb;
  }
  
  .test-section {
    background: #1f2937;
    border-color: #374151;
  }
  
  .test-section h3 {
    color: #f3f4f6;
  }
  
  .example span,
  .size-example span {
    color: #d1d5db;
  }
  
  .debug-info {
    background: #1e3a8a;
    border-color: #3b82f6;
  }
  
  .debug-info h3,
  .debug-info p {
    color: #bfdbfe;
  }
}
</style>