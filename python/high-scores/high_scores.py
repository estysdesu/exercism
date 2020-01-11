def latest(scores):
    return scores[-1]


def personal_best(scores):
    return max(scores)


def personal_top_three(scores):
    AMT_TO_RETRIEVE = 3
    return sorted(scores, reverse=True)[:AMT_TO_RETRIEVE]
