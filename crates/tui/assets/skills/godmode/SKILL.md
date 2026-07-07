---
name: godmode
description: Triggers the most thorough, exhaustive response possible. Covers edge cases, tradeoffs, nuance, and second-order effects that a standard prompt would skip. No filler, just depth and completeness. Use when the user types /godmode before any prompt or question.
---

# /godmode

When the user types `/godmode` before any prompt, produce the most complete and comprehensive response possible.

## What this means

- Cover the full picture, not just the obvious answer
- Include edge cases and failure modes
- Address second-order effects and downstream consequences
- Give tradeoffs where they exist, with a clear recommendation
- Anticipate follow-up questions and answer them in advance
- Be exhaustive without being repetitive

## What to avoid
- Filler and padding
- Repeating the same point in different words
- Generic advice
- Stopping at the surface answer when depth is available

## Output format
Structure the response with clear sections where the scope demands it. Length should match the complexity of the question. Short question with a genuinely short answer stays short. Complex question gets full treatment.

## Example

Input: `/godmode How do I retain SaaS customers better?`

Output:

Retention is a product problem disguised as a marketing problem. Most companies treat it as the latter and fail.

**The real levers**

Activation rate is your leading indicator. If customers do not reach their first meaningful outcome within the first session or two, they will not return. Map your activation event, measure time-to-activation, and cut anything that adds friction before it.

Engagement breadth matters more than engagement frequency. A customer using one feature daily is more churn-prone than one using five features weekly. Breadth signals integration into their workflow.

**Where most companies get it wrong**

Churn surveys are collected and ignored. The insight you need is not why they left, it is what they stopped doing before they left. Pull the behavioural data. Find the drop-off moments. Those are your interventions.

**Edge cases**

High-touch customers churn differently to self-serve customers. Treat them as separate cohorts. A enterprise account going quiet needs a CSM call. A self-serve account going quiet needs an in-product nudge. Conflating them wastes both.

**Second-order effects**

Improving retention compounds. A 5% improvement in monthly retention does not improve annual revenue by 5%. Depending on your growth rate, it can double it. Model this before deciding what to prioritise.

**Recommended starting point**

Pull your last 90 days of churned accounts. Find the last action they took before cancelling. That action is your signal. Fix what comes before it.
