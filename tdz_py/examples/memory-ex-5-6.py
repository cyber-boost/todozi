#!/usr/bin/env python3
"""
Example 5: Memory Management and Emotional Tracking System
Demonstrates creating various types of memories and tracking emotional states across tasks and events.
"""

import asyncio
from datetime import datetime, timezone
from todozi.memory import Memory, MemoryManager, EmotionalMemoryType, MemoryImportance, MemoryTerm
from todozi.tags import TagManager

class EmotionalMemoryTracker:
    """Tracks emotional states across tasks and creates emotional memories."""
    
    def __init__(self, memory_manager: MemoryManager, tag_manager: TagManager):
        self.memory_manager = memory_manager
        self.tag_manager = tag_manager
        self.emotion_timeline = []
    
    async def create_emotional_memory(self, moment: str, meaning: str, emotion: str, 
                                    intensity: int = 5, context: str = "general") -> str:
        """Create an emotional memory with intensity tracking."""
        
        try:
            # Validate emotion
            emotional_type = EmotionalMemoryType(emotion.lower())
            
            # Create memory with emotional context
            memory = Memory(
                user_id="emotional_tracker",
                moment=f"[Intensity {intensity}/10] {moment}",
                meaning=f"{meaning} | Emotion: {emotion} | Context: {context}",
                reason="Emotional state tracking",
                importance=MemoryImportance.Medium,
                term=MemoryTerm.Long,
                memory_type=emotional_type,
                tags=[f"emotion_{emotion}", f"intensity_{intensity}", context.lower()]
            )
            
            # Add to memory manager
            memory_id = await self.memory_manager.create_memory(memory)
            
            # Track in timeline
            self.emotion_timeline.append({
                'timestamp': datetime.now(timezone.utc),
                'emotion': emotion,
                'intensity': intensity,
                'memory_id': memory_id,
                'context': context
            })
            
            # Update tag usage for emotion tracking
            await self.tag_manager.increment_tag_usage(f"emotion_{emotion}")
            
            return memory_id
            
        except ValueError as e:
            print(f"❌ Invalid emotion '{emotion}': {e}")
            return None

    async def create_task_emotional_snapshot(self, task_description: str, task_completion: bool, 
                                           emotions: list, overall_mood: str) -> str:
        """Create a comprehensive emotional snapshot related to a specific task."""
        
        emotional_summary = "; ".join(emotions)
        
        memory = Memory(
            user_id="task_emotional_tracker",
            moment=f"Task: {task_description} | Completion: {task_completion}",
            meaning=f"Emotional journey: {emotional_summary} | Overall mood: {overall_mood}",
            reason="Task emotional analysis",
            importance=MemoryImportance.High,
            term=MemoryTerm.Long,
            memory_type=EmotionalMemoryType(overall_mood.lower()),
            tags=["task_emotion", "emotional_snapshot", f"completed_{task_completion}"] + emotions
        )
        
        return await self.memory_manager.create_memory(memory)

    async def get_emotion_statistics(self) -> dict:
        """Get statistics about emotional patterns."""
        
        emotions = {}
        intensity_sum = 0
        intensity_count = 0
        
        for entry in self.emotion_timeline:
            emotion = entry['emotion']
            intensity = entry['intensity']
            
            if emotion not in emotions:
                emotions[emotion] = {
                    'count': 0,
                    'total_intensity': 0,
                    'contexts': set()
                }
            
            emotions[emotion]['count'] += 1
            emotions[emotion]['total_intensity'] += intensity
            emotions[emotion]['contexts'].add(entry['context'])
            intensity_sum += intensity
            intensity_count += 1
        
        stats = {
            'total_emotional_memories': len(self.emotion_timeline),
            'emotions': {},
            'average_intensity': intensity_sum / intensity_count if intensity_count > 0 else 0
        }
        
        for emotion, data in emotions.items():
            stats['emotions'][emotion] = {
                'frequency': data['count'],
                'average_intensity': data['total_intensity'] / data['count'],
                'contexts': list(data['contexts'])
            }
        
        return stats

    async def find_patterns(self) -> dict:
        """Identify emotional patterns across different contexts."""
        
        patterns = {
            'high_intensity_contexts': {},
            'frequent_emotions_by_context': {},
            'emotional_transitions': []
        }
        
        # Group by context
        context_emotions = {}
        for entry in self.emotion_timeline:
            context = entry['context']
            emotion = entry['emotion']
            intensity = entry['intensity']
            
            if context not in context_emotions:
                context_emotions[context] = []
            
            context_emotions[context].append((emotion, intensity))
        
        # Find high intensity contexts
        for context, emotions in context_emotions.items():
            if len(emotions) >= 3:  # Only analyze contexts with sufficient data
                avg_intensity = sum(intensity for _, intensity in emotions) / len(emotions)
                if avg_intensity >= 7:
                    patterns['high_intensity_contexts'][context] = {
                        'average_intensity': avg_intensity,
                        'sample_size': len(emotions)
                    }
        
        # Find frequent emotions by context
        for context, emotions in context_emotions.items():
            emotion_counts = {}
            for emotion, _ in emotions:
                emotion_counts[emotion] = emotion_counts.get(emotion, 0) + 1
            
            if emotion_counts:
                most_frequent = max(emotion_counts.items(), key=lambda x: x[1])
                patterns['frequent_emotions_by_context'][context] = {
                    'emotion': most_frequent[0],
                    'frequency': most_frequent[1]
                }
        
        return patterns


async def main():
    """Demo the emotional memory tracking system."""
    
    print("🧠 Emotional Memory Management System")
    print("=" * 50)
    
    # Initialize managers
    memory_manager = MemoryManager()
    tag_manager = TagManager()
    emotional_tracker = EmotionalMemoryTracker(memory_manager, tag_manager)
    
    # Create various types of memories
    print("\n📝 Creating Different Memory Types:")
    
    # 1. Standard memory (factual)
    standard_memory = Memory(
        user_id="demo_user",
        moment="Completed project documentation",
        meaning="Thorough documentation improves maintainability",
        reason="Project completion milestone",
        importance=MemoryImportance.High,
        term=MemoryTerm.Long,
        memory_type="Standard",
        tags=["documentation", "project", "milestone"]
    )
    await memory_manager.create_memory(standard_memory)
    print("✅ Standard memory created")
    
    # 2. Secret memory (AI-only)
    secret_memory = Memory(
        user_id="demo_user",
        moment="System performance optimization insights",
        meaning="Identified critical bottlenecks in database queries",
        reason="Internal optimization analysis",
        importance=MemoryImportance.Critical,
        term=MemoryTerm.Long,
        memory_type="Secret",
        tags=["optimization", "performance", "internal"]
    )
    await memory_manager.create_memory(secret_memory)
    print("✅ Secret memory created (AI-only)")
    
    # 3. Emotional memories
    emotions_to_track = [
        ("Successful deployment", "Excited about smooth deployment process", "excited", 8, "deployment"),
        ("Code review feedback", "Frustrated with unclear feedback", "frustrated", 6, "code_review"),
        ("Team collaboration", "Happy about effective team communication", "happy", 7, "teamwork"),
        ("Deadline pressure", "Anxious about tight deadline", "anxious", 9, "deadline"),
        ("Learning breakthrough", "Motivated by understanding complex concept", "motivated", 8, "learning")
    ]
    
    print("\n😊 Tracking Emotional States:")
    for moment, meaning, emotion, intensity, context in emotions_to_track:
        memory_id = await emotional_tracker.create_emotional_memory(
            moment, meaning, emotion, intensity, context
        )
        if memory_id:
            print(f"✅ Emotional memory: {emotion} (intensity {intensity}) - {moment[:30]}...")
    
    # 4. Task emotional snapshot
    task_snapshot_id = await emotional_tracker.create_task_emotional_snapshot(
        "Implement authentication system",
        True,
        ["frustrated", "determined", "satisfied"],
        "proud"
    )
    print(f"✅ Task emotional snapshot created (ID: {task_snapshot_id})")
    
    # Analyze emotional patterns
    print("\n📊 Emotional Analysis:")
    stats = await emotional_tracker.get_emotion_statistics()
    print(f"Total emotional memories: {stats['total_emotional_memories']}")
    print(f"Average intensity: {stats['average_intensity']:.1f}/10")
    
    print("\nEmotion breakdown:")
    for emotion, data in stats['emotions'].items():
        print(f"  {emotion}: {data['frequency']} occurrences, avg intensity {data['average_intensity']:.1f}")
    
    # Find patterns
    patterns = await emotional_tracker.find_patterns()
    if patterns['high_intensity_contexts']:
        print("\n🔥 High Intensity Contexts:")
        for context, data in patterns['high_intensity_contexts'].items():
            print(f"  {context}: intensity {data['average_intensity']:.1f} (based on {data['sample_size']} samples)")
    
    # Search for emotional memories
    print("\n🔍 Searching Emotional Memories:")
    excited_memories = memory_manager.get_memories_by_type(EmotionalMemoryType("excited"))
    print(f"Found {len(excited_memories)} 'excited' memories")
    
    for memory in excited_memories[:2]:  # Show first 2
        print(f"  - {memory.moment[:40]}...")
    
    # Memory statistics
    print("\n📈 Memory System Statistics:")
    mem_stats = memory_manager.get_memory_statistics()
    print(f"Total memories: {mem_stats.total_memories}")
    print(f"Emotional memories: {mem_stats.emotional_memories}")
    print(f"Short-term: {mem_stats.short_term_memories} ({mem_stats.short_term_percentage:.1f}%)")
    print(f"Long-term: {mem_stats.long_term_memories} ({mem_stats.long_term_percentage:.1f}%)")
    print(f"Critical: {mem_stats.critical_memories} ({mem_stats.critical_percentage:.1f}%)")
    
    # Tag statistics
    tag_stats = memory_manager.get_tag_statistics()
    emotional_tags = {tag: count for tag, count in tag_stats.items() if tag.startswith('emotion_')}
    if emotional_tags:
        print("\n🏷️ Emotional Tag Usage:")
        for tag, count in sorted(emotional_tags.items(), key=lambda x: x[1], reverse=True)[:3]:
            print(f"  {tag}: {count} memories")


if __name__ == "__main__":
    asyncio.run(main())